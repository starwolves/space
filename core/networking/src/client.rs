use std::{
    collections::BTreeMap,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{
    ecs::{schedule::SystemSet, system::Local},
    log::error,
};
use bevy::{log::info, time::Time};
use bevy::{
    prelude::{Event, Resource},
    tasks::{AsyncComputeTaskPool, Task},
};

use bevy_renet::renet::{
    transport::{
        ClientAuthentication, ConnectToken, NetcodeClientTransport, NetcodeTransportError,
    },
    ConnectionConfig, DefaultChannel, RenetClient,
};
use bevy_xpbd_3d::plugins::setup::{Physics, PhysicsTime};
use futures_lite::future;
use resources::core::TickRate;
use token::parse::Token;

use crate::{
    messaging::{
        ReliableClientMessageBatch, ReliableMessage, UnreliableClientMessageBatch,
        UnreliableMessage,
    },
    plugin::{RENET_RELIABLE_ORDERED_ID, RENET_RELIABLE_UNORDERED_ID},
    server::{DEFAULT_MIN_LATENCY, DEFAULT_MIN_REQUIRED_MESSAGES_FOR_ADJUSTMENT, PROTOCOL_ID},
    stamp::TickRateStamp,
};

/// Resource containing needed for the server.

#[derive(Default, Resource, Clone)]
pub struct ConnectionPreferences {
    pub account_name: String,
    pub server_address: String,
}

/// Event that triggers a new server connection.
#[derive(Event)]
pub struct AssignTokenToServer;
#[derive(Event)]
pub struct ConnectToServer;

use crate::server::SERVER_PORT;
use bevy::log::warn;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::Res;
use std::net::IpAddr;

use bevy::prelude::ResMut;

use crate::server::PRIV_KEY;

#[derive(Resource, Default)]
pub struct AssigningServerToken {
    pub bool: bool,
}

pub fn token_assign_server(
    mut events: EventReader<AssignTokenToServer>,
    mut commands: Commands,
    token: Res<Token>,
    preferences: Res<ConnectionPreferences>,
    mut state: ResMut<AssigningServerToken>,
) {
    for _ in events.read() {
        if state.bool {
            continue;
        }
        state.bool = true;
        let data = vec![
            ("token", token.token.clone()),
            ("serverAddress", preferences.server_address.clone()),
        ];

        let x = TokenAssignServer {
            task: AsyncComputeTaskPool::get().spawn(async move {
                let encoded = form_urlencoded::Serializer::new(String::new())
                    .extend_pairs(data)
                    .finish();

                let mut post = ehttp::Request::post(
                    format!("https://store.starwolves.io/token_assign_server"),
                    encoded.into_bytes(),
                );
                post.headers = ehttp::Headers::new(&[
                    ("Accept", "*/*"),
                    (
                        "Content-Type",
                        "application/x-www-form-urlencoded; charset=utf-8",
                    ),
                ]);
                ehttp::fetch_blocking(&post).expect("Error with HTTP call")
            }),
        };

        commands.insert_resource(x);
    }
}

#[derive(Serialize, Deserialize)]
struct Response {
    pub valid: bool,
}

#[derive(Resource)]
pub struct TokenAssignServer {
    pub task: Task<ehttp::Response>,
}
pub fn starwolves_response(
    mut commands: Commands,
    mut task: ResMut<TokenAssignServer>,
    mut connect: EventWriter<ConnectToServer>,
    mut state: ResMut<AssigningServerToken>,
) {
    if let Some(response) = future::block_on(future::poll_once(&mut task.task)) {
        // Process the response
        match serde_json::from_slice::<Response>(response.bytes.as_slice()) {
            Ok(d) => {
                if !d.valid {
                    warn!("Invalid token. Log in with the launcher then restart the game. [https://store.starwolves.io]");
                } else {
                    connect.send(ConnectToServer);
                    info!("[Starwolves.io] Token assigned to new connection.");
                }
            }
            Err(e) => {
                error!("Unexpected response: {:?}", e);
            }
        }

        // Dispose of the consumed HTTP Call by deleting the Entity from ECS
        commands.remove_resource::<TokenAssignServer>();
        state.bool = false;
    }
}

use std::convert::TryInto;

fn convert<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub(crate) fn connect_to_server(
    mut event: EventReader<ConnectToServer>,
    mut commands: Commands,
    preferences: Res<ConnectionPreferences>,
    mut connection_state: ResMut<Connection>,
    token: Res<Token>,
) {
    for _ in event.read() {
        match connection_state.status {
            ConnectionStatus::None => {
                info!("Initializing connection with server.");
                let address;
                let port;

                match preferences.server_address.split_once(":") {
                    Some((ip, port_str)) => {
                        address = ip;
                        match port_str.parse::<u16>() {
                            Ok(p) => {
                                port = p;
                            }
                            Err(_rr) => {
                                warn!("Couldn't connect: couldn't parse port.");
                                continue;
                            }
                        };
                    }
                    None => {
                        address = &preferences.server_address;
                        port = SERVER_PORT
                    }
                }

                let ip_address;

                match address.parse::<IpAddr>() {
                    Ok(add) => {
                        ip_address = add;
                    }
                    Err(_) => {
                        warn!("Couldn't connect: invalid server address.");
                        continue;
                    }
                }
                let socket;
                match UdpSocket::bind(local_ipaddress::get().unwrap_or_default() + ":0") {
                    Ok(s) => {
                        socket = s;
                    }
                    Err(err) => {
                        warn!("Failed to bind udp socket: {}", err);
                        continue;
                    }
                }

                let socket_address: SocketAddr = SocketAddr::new(ip_address, port as u16);

                let channels_config = DefaultChannel::config();
                let current_time = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                let client_id = current_time.as_millis() as u64;

                info!("Connecting to {}...", socket_address);

                let token = token.token.as_bytes();
                let token_sized: &[u8; 256] = &convert(token.to_vec());

                match ConnectToken::generate(
                    current_time,
                    PROTOCOL_ID,
                    120,
                    client_id,
                    120,
                    vec![socket_address],
                    Some(token_sized),
                    &PRIV_KEY,
                ) {
                    Ok(connect_token) => {
                        let renet_client = RenetClient::new(ConnectionConfig {
                            server_channels_config: channels_config.clone(),
                            client_channels_config: channels_config,
                            ..Default::default()
                        });

                        let authentication = ClientAuthentication::Secure { connect_token };

                        let transport =
                            NetcodeClientTransport::new(current_time, authentication, socket)
                                .unwrap();

                        commands.insert_resource(renet_client);
                        commands.insert_resource(transport);

                        connection_state.status = ConnectionStatus::Connecting;
                    }
                    Err(err) => {
                        warn!("Token generation failed: {:?}", err);
                    }
                }
            }
            ConnectionStatus::Connecting => {
                continue;
            }
            ConnectionStatus::Connected => {
                continue;
            }
        }
    }
}

#[derive(Default, Resource)]
pub struct Connection {
    pub status: ConnectionStatus,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ConnectionStatus {
    #[default]
    None,
    Connecting,
    Connected,
}

use bevy::prelude::EventWriter;

/// System run run_if

pub fn connected(connection: Res<Connection>) -> bool {
    matches!(connection.status, ConnectionStatus::Connected)
}
/// System run run_if. The earliest server messages (for setup_ui, boarding etc.)
/// come in while in the connecting stage.

pub fn is_client_connected(connection: Res<Connection>) -> bool {
    matches!(connection.status, ConnectionStatus::Connecting)
        || matches!(connection.status, ConnectionStatus::Connected)
}
use crate::messaging::ReliableServerMessageBatch;
use crate::messaging::Typenames;
use crate::plugin::RENET_UNRELIABLE_CHANNEL_ID;

use serde::Serialize;
use typename::TypeName;

#[derive(Resource, Default)]
pub(crate) struct OutgoingBuffer {
    pub reliable: Vec<ReliableMessage>,
    pub reliable_unordered: Vec<ReliableMessage>,
    pub unreliable: Vec<UnreliableMessage>,
}

pub(crate) fn step_buffer(
    mut res: ResMut<OutgoingBuffer>,
    mut client: ResMut<RenetClient>,
    stamp: Res<TickRateStamp>,
    pause_loop: Res<Time<Physics>>,
) {
    if res.reliable.len() > 0 {
        let bin;
        match bincode::serialize(&ReliableClientMessageBatch {
            messages: res.reliable.clone(),
            stamp: stamp.tick,
            fixed: pause_loop.is_paused(),
        }) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize step_buffer message");
                return;
            }
        }

        client.send_message(RENET_RELIABLE_ORDERED_ID, bin);
    }
    if res.reliable_unordered.len() > 0 {
        let bin;
        match bincode::serialize(&ReliableClientMessageBatch {
            messages: res.reliable_unordered.clone(),
            stamp: stamp.tick,
            fixed: pause_loop.is_paused(),
        }) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize step_buffer message");
                return;
            }
        }

        client.send_message(RENET_RELIABLE_UNORDERED_ID, bin);
    }
    if res.unreliable.len() > 0 {
        let bin;
        match bincode::serialize(&UnreliableClientMessageBatch {
            messages: res.unreliable.clone(),
            stamp: stamp.tick,
            fixed: pause_loop.is_paused(),
        }) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize step_buffer message");
                return;
            }
        }

        client.send_message(RENET_UNRELIABLE_CHANNEL_ID, bin);
    }

    res.reliable.clear();
    res.unreliable.clear();
    res.reliable_unordered.clear();
}

/// Serializes and sends the outgoing reliable client messages.
pub(crate) fn send_outgoing_reliable_client_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingReliableClientMessage<T>>,
    mut client: ResMut<OutgoingBuffer>,
    typenames: Res<Typenames>,
) {
    for message in events.read() {
        let net;
        match typenames
            .reliable_net_types
            .get(&message.message.type_name_of())
        {
            Some(n) => {
                net = n;
            }
            None => {
                warn!("Couldnt find client reliable type");
                continue;
            }
        }
        let bin;
        match bincode::serialize(&message.message) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize reliable message");
                continue;
            }
        }
        let mut unordered = false;
        for x in &typenames.reliable_unordered_types {
            if x == &message.message.type_name_of() {
                unordered = true;
                break;
            }
        }

        if unordered {
            client.reliable_unordered.push(ReliableMessage {
                serialized: bin,
                typename_net: *net,
            });
        } else {
            client.reliable.push(ReliableMessage {
                serialized: bin,
                typename_net: *net,
            });
        }
    }
}
use crate::messaging::UnreliableServerMessageBatch;

/// Serializes and sends the outgoing unreliable client messages.
pub(crate) fn send_outgoing_unreliable_client_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingUnreliableClientMessage<T>>,
    mut client: ResMut<OutgoingBuffer>,
    typenames: Res<Typenames>,
) {
    for message in events.read() {
        let net;

        match typenames
            .unreliable_net_types
            .get(&message.message.type_name_of())
        {
            Some(n) => {
                net = n;
            }
            None => {
                warn!("Couldnt find unreliable type");
                continue;
            }
        }
        let bin;
        match bincode::serialize(&message.message) {
            Ok(b) => {
                bin = b;
            }
            Err(_) => {
                warn!("Couldnt serialize unreliable message");
                continue;
            }
        }

        client.unreliable.push(UnreliableMessage {
            serialized: bin,
            typename_net: *net,
        });
    }
}
pub(crate) fn deserialize_incoming_unreliable_server_message<
    T: Clone + TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawUnreliableServerMessage>,
    mut outgoing: EventWriter<IncomingUnreliableServerMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    for batch in incoming_raw.read() {
        let server_stamp = stamp.calculate_large(batch.message.stamp);

        for message in batch.message.messages.iter() {
            match get_unreliable_message::<T>(&typenames, message.typename_net, &message.serialized)
            {
                Some(data) => {
                    let r = IncomingUnreliableServerMessage {
                        message: data,
                        stamp: server_stamp,
                    };
                    outgoing.send(r.clone());
                }
                None => {}
            }
        }
    }
}
use crate::messaging::get_reliable_message;

#[derive(Resource, Default)]
pub struct QueuedSpawnEntityRaw {
    pub reliable: Vec<IncomingRawReliableServerMessage>,
    pub unreliable: Vec<IncomingRawUnreliableServerMessage>,
}
pub(crate) fn clear_raw_spawn_entity_queue(mut incoming_raw: ResMut<QueuedSpawnEntityRaw>) {
    incoming_raw.reliable.clear();
    incoming_raw.unreliable.clear();
}
// Deserializes messages a second time, this time EntityUpdates contained in LoadEntity call.
pub fn deserialize_incoming_reliable_load_entity_updates<
    T: Clone + TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    incoming_raw: Res<QueuedSpawnEntityRaw>,
    mut outgoing: EventWriter<IncomingReliableServerMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    for batch in incoming_raw.reliable.iter() {
        let server_stamp = stamp.calculate_large(batch.message.stamp);

        for message in batch.message.messages.iter() {
            match get_reliable_message::<T>(&typenames, message.typename_net, &message.serialized) {
                Some(data) => {
                    let r = IncomingReliableServerMessage {
                        message: data,
                        stamp: server_stamp,
                    };
                    outgoing.send(r);
                }
                None => {}
            }
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct BevyPreUpdateSendMessage;
/// Latency critical input gets fired from Bevy's PreUpdate schedule rather than in PostUpdate.
pub(crate) fn pre_update_send_messages(
    mut transport: ResMut<NetcodeClientTransport>,
    mut client: ResMut<RenetClient>,
    mut transport_errors: EventWriter<NetcodeTransportError>,
) {
    if let Err(e) = transport.send_packets(&mut client) {
        transport_errors.send(e);
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct PostUpdateSendMessage;
pub(crate) fn post_update_send_messages(
    mut transport: ResMut<NetcodeClientTransport>,
    mut client: ResMut<RenetClient>,
    mut transport_errors: EventWriter<NetcodeTransportError>,
) {
    if let Err(e) = transport.send_packets(&mut client) {
        transport_errors.send(e);
    }
}
/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct DeserializeSpawnUpdates;
pub fn deserialize_incoming_unreliable_load_entity_updates<
    T: Clone + TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    incoming_raw: Res<QueuedSpawnEntityRaw>,
    mut outgoing: EventWriter<IncomingUnreliableServerMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    for batch in incoming_raw.unreliable.iter() {
        let server_stamp = stamp.calculate_large(batch.message.stamp);

        for message in batch.message.messages.iter() {
            match get_unreliable_message::<T>(&typenames, message.typename_net, &message.serialized)
            {
                Some(data) => {
                    let r = IncomingUnreliableServerMessage {
                        message: data,
                        stamp: server_stamp,
                    };
                    outgoing.send(r);
                }
                None => {}
            }
        }
    }
}

pub fn deserialize_incoming_reliable_server_message<
    T: Clone + TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawReliableServerMessage>,
    mut outgoing: EventWriter<IncomingReliableServerMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    for batch in incoming_raw.read() {
        let server_stamp = stamp.calculate_large(batch.message.stamp);

        for message in batch.message.messages.iter() {
            match get_reliable_message::<T>(&typenames, message.typename_net, &message.serialized) {
                Some(data) => {
                    let r = IncomingReliableServerMessage {
                        message: data,
                        stamp: server_stamp,
                    };
                    outgoing.send(r.clone());
                }
                None => {}
            }
        }
    }
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event, Clone)]
pub struct IncomingReliableServerMessage<T: TypeName + Send + Sync + Serialize> {
    pub message: T,
    pub stamp: u64,
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event, Clone)]
pub struct IncomingUnreliableServerMessage<T: TypeName + Send + Sync + Serialize> {
    pub message: T,
    pub stamp: u64,
}

/// Dezerializes incoming server messages and writes to event.

pub(crate) fn receive_incoming_unreliable_server_messages(
    mut events: EventWriter<IncomingRawUnreliableServerMessage>,
    mut client: ResMut<RenetClient>,
    mut queue: Local<BTreeMap<u64, Vec<IncomingRawUnreliableServerMessage>>>,
    stamp: Res<TickRateStamp>,
    latency: Res<TickLatency>,
) {
    while let Some(msg) = client.receive_message(RENET_UNRELIABLE_CHANNEL_ID) {
        match bincode::deserialize::<UnreliableServerMessageBatch>(&msg) {
            Ok(message) => {
                let store_stamp = stamp.calculate_large(message.stamp);
                let r = IncomingRawUnreliableServerMessage { message: message };
                match queue.get_mut(&store_stamp) {
                    Some(v) => {
                        v.push(r);
                    }
                    None => {
                        queue.insert(store_stamp, vec![r]);
                    }
                }
            }
            Err(_) => {
                warn!("Received an invalid message 0.");
            }
        }
    }
    let latency_in_ticks = latency.latency as u64;
    let desired_tick = stamp.large - latency_in_ticks;
    let bound_queue = queue.clone();
    let mut is = vec![];
    // Messages are either main FixedUpdate reliable batches or separated small message batches sent from Update for low latency input replication.
    for i in bound_queue.keys() {
        if *i > desired_tick {
            break;
        }
        let msgs = queue.get(i).unwrap();

        for m in msgs {
            events.send(m.clone());
        }
        is.push(*i);
    }
    for i in is {
        queue.remove(&i);
    }
}

/// Deserializes incoming server messages and writes to event.

pub(crate) fn receive_incoming_reliable_server_messages(
    mut client: ResMut<RenetClient>,
    mut events: EventWriter<IncomingRawReliableServerMessage>,
    mut queue: Local<BTreeMap<u64, Vec<IncomingRawReliableServerMessage>>>,
    stamp: Res<TickRateStamp>,
    latency: Res<TickLatency>,
) {
    while let Some(msg) = client.receive_message(RENET_RELIABLE_ORDERED_ID) {
        match bincode::deserialize::<ReliableServerMessageBatch>(&msg) {
            Ok(message) => {
                let server_stamp = stamp.calculate_large(message.stamp);
                let r = IncomingRawReliableServerMessage {
                    message: message.clone(),
                };

                match queue.get_mut(&server_stamp) {
                    Some(v) => {
                        v.push(r);
                    }
                    None => {
                        queue.insert(server_stamp, vec![r]);
                    }
                }
                info!(
                    "Queng to fire message batch server stamp ({}, d={}) {} at {}.",
                    message.stamp,
                    stamp.get_difference(message.stamp),
                    server_stamp,
                    stamp.large,
                );
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
    }
    while let Some(msg) = client.receive_message(RENET_RELIABLE_UNORDERED_ID) {
        match bincode::deserialize::<ReliableServerMessageBatch>(&msg) {
            Ok(message) => {
                let server_stamp = stamp.calculate_large(message.stamp);
                let r = IncomingRawReliableServerMessage { message: message };

                match queue.get_mut(&server_stamp) {
                    Some(v) => {
                        v.push(r);
                    }
                    None => {
                        queue.insert(server_stamp, vec![r]);
                    }
                }
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
    }

    let latency_in_ticks = latency.latency as u64;
    let desired_tick = stamp.large - latency_in_ticks;
    let bound_queue = queue.clone();
    let mut is = vec![];
    // Process one message batch per tick.
    for i in bound_queue.keys() {
        if *i > desired_tick {
            break;
        }
        let msgs = queue.get(i).unwrap();

        for m in msgs {
            events.send(m.clone());
        }
        is.push(*i);

        info!(
            "Firing message batch at tick {}, server stamp {}.",
            stamp.large, i
        );
        break;
    }

    for i in is {
        queue.remove(&i);
    }
}

/// Event to send unreliable messages from client to server.
#[derive(Event)]
pub struct OutgoingUnreliableClientMessage<T: TypeName + Send + Sync + 'static> {
    pub message: T,
}
/// Event to send reliable messages from client to server.

#[derive(Event)]
pub struct OutgoingReliableClientMessage<T: TypeName + Send + Sync + 'static> {
    pub message: T,
}

/// Event to when received reliable message from server. Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event, Clone)]
pub struct IncomingRawReliableServerMessage {
    pub message: ReliableServerMessageBatch,
}

/// Event to when received reliable message from server. Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event, Clone)]
pub struct IncomingRawUnreliableServerMessage {
    pub message: UnreliableServerMessageBatch,
}

/// Returns an option containing the desired unreliable netcode message.

pub fn get_unreliable_message<T: TypeName + Serialize + for<'a> Deserialize<'a>>(
    typenames: &Res<Typenames>,
    identifier: u8,
    message: &[u8],
) -> Option<T> {
    match typenames.unreliable_net_types.get(&T::type_name()) {
        Some(i) => {
            if &identifier == i {
                match bincode::deserialize::<T>(message) {
                    Ok(t) => Some(t),
                    Err(_) => {
                        warn!("Couldnt serialize message!");
                        None
                    }
                }
            } else {
                None
            }
        }
        None => {
            warn!("Couldnt find reliable net type.");
            None
        }
    }
}

/// Total latency including fps drop compensations. Not reflective of network latency.
#[derive(Resource, Default)]
pub struct TotalAdjustment {
    pub latency: i16,
}
use crate::server::NetworkingServerMessage;

// Waits this amount of ticks per sync message send at 60hz.
const TEST_MESSAGES_FREQUENCY: f32 = 4.;

pub(crate) fn sync_check_client(
    mut net: EventWriter<OutgoingUnreliableClientMessage<NetworkingUnreliableClientMessage>>,
    mut skip: Local<u8>,
    rate: Res<TickRate>,
) {
    if *skip == u8::MAX {
        *skip = 0;
    } else {
        *skip += 1;
    }
    if *skip > (TEST_MESSAGES_FREQUENCY * (rate.fixed_rate as f32 / 60.)).round() as u8 {
        net.send(OutgoingUnreliableClientMessage {
            message: NetworkingUnreliableClientMessage::HeartBeat,
        });
        *skip = 0;
    }
}

#[derive(Event)]
pub struct ClientGameWorldLoaded;

#[derive(Resource, Default)]
pub struct LoadedGameWorldBuffer(pub bool);

pub fn detect_client_world_loaded(
    mut state: ResMut<LoadedGameWorldBuffer>,
    mut events: EventWriter<ClientGameWorldLoaded>,
    mut out: EventWriter<OutgoingReliableClientMessage<NetworkingClientMessage>>,
) {
    if state.0 {
        state.0 = false;
        events.send(ClientGameWorldLoaded);
        out.send(OutgoingReliableClientMessage {
            message: NetworkingClientMessage::LoadedGameWorld,
        });
    }
}

/// Confirms connection with server.
pub(crate) fn confirm_connection(
    mut client1: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut connected_state: ResMut<Connection>,
) {
    for message in client1.read() {
        let player_message = message.message.clone();
        match player_message {
            NetworkingServerMessage::Awoo => {
                connected_state.status = ConnectionStatus::Connected;
                info!("Connected to game server.");
            }
            _ => (),
        }
    }
}

pub(crate) fn on_disconnect(
    client: Res<RenetClient>,
    mut connected_state: ResMut<Connection>,
    mut commands: Commands,
) {
    match client.is_disconnected() {
        true => {
            warn!(
                "Disconnected from server: [{:?}]",
                client.disconnect_reason()
            );
            connected_state.status = ConnectionStatus::None;
            commands.remove_resource::<RenetClient>();
        }
        false => {}
    }
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum NetworkingClientMessage {
    SyncConfirmation,
    LoadedGameWorld,
    StartSyncConfirmation,
}
/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum NetworkingUnreliableClientMessage {
    HeartBeat,
}

use serde::Deserialize;

/// Client resource. Gives latency in ticks.
#[derive(Resource)]
pub struct TickLatency {
    pub latency: u16,
    buffer: Vec<u16>,
}

impl Default for TickLatency {
    fn default() -> Self {
        Self {
            latency: DEFAULT_MIN_LATENCY as u16 * 2,
            buffer: vec![],
        }
    }
}

pub(crate) fn update_tick_latency(
    mut latency: ResMut<TickLatency>,
    client: Res<RenetClient>,
    tickrate: Res<TickRate>,
) {
    let latency_in_ticks = (client.rtt() as f32 / (1. / tickrate.fixed_rate as f32))
        .round()
        .clamp(DEFAULT_MIN_LATENCY as f32 * 2., f32::MAX) as u16;
    latency.buffer.push(latency_in_ticks);
    if latency.buffer.len() > DEFAULT_MIN_REQUIRED_MESSAGES_FOR_ADJUSTMENT as usize {
        latency.buffer.remove(0);
    }
    let mut total = 0;
    for i in &latency.buffer {
        total += i;
    }
    latency.latency = total / latency.buffer.len() as u16;
}
