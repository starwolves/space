use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{ecs::system::Local, log::error};
use bevy::{log::info, time::Time};
use bevy::{
    prelude::{Event, Resource},
    tasks::{AsyncComputeTaskPool, Task},
};

use bevy_renet::renet::{
    transport::{ClientAuthentication, ConnectToken, NetcodeClientTransport},
    ConnectionConfig, DefaultChannel, RenetClient,
};
use bevy_xpbd_3d::plugins::setup::{Physics, PhysicsTime};
use futures_lite::future;
use itertools::Itertools;
use resources::core::TickRate;
use token::parse::Token;

use crate::{
    messaging::{
        ReliableClientMessageBatch, ReliableMessage, UnreliableClientMessageBatch,
        UnreliableMessage,
    },
    plugin::{RENET_RELIABLE_ORDERED_ID, RENET_RELIABLE_UNORDERED_ID},
    server::PROTOCOL_ID,
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
                post.headers = ehttp::headers(&[
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
                    warn!("Invalid token. Log in with the launcher. Try restarting it.");
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
            not_timed: pause_loop.is_paused(),
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
            not_timed: pause_loop.is_paused(),
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
            not_timed: pause_loop.is_paused(),
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
use serde::Deserialize;

pub(crate) fn deserialize_incoming_unreliable_server_message<
    T: Clone + TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawUnreliableServerMessage>,
    mut outgoing: EventWriter<IncomingUnreliableServerMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    for event in incoming_raw.read() {
        for message in event.message.messages.iter() {
            match get_unreliable_message::<T>(&typenames, message.typename_net, &message.serialized)
            {
                Some(data) => {
                    let b = stamp.calculate_large(event.message.stamp);

                    let r = IncomingUnreliableServerMessage {
                        message: data,
                        stamp: b,
                    };
                    outgoing.send(r.clone());
                }
                None => {}
            }
        }
    }
}
use crate::messaging::get_reliable_message;

pub fn deserialize_incoming_reliable_server_message<
    T: Clone + TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawReliableServerMessage>,
    mut outgoing: EventWriter<IncomingReliableServerMessage<T>>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
    mut queue: Local<HashMap<u64, Vec<IncomingReliableServerMessage<T>>>>,
) {
    for event in incoming_raw.read() {
        for message in event.message.messages.iter() {
            match get_reliable_message::<T>(&typenames, message.typename_net, &message.serialized) {
                Some(data) => {
                    let b = stamp.calculate_large(event.message.stamp);

                    let r = IncomingReliableServerMessage {
                        message: data,
                        stamp: b,
                    };

                    match queue.get_mut(&b) {
                        Some(v) => {
                            v.push(r);
                        }
                        None => {
                            queue.insert(b, vec![r]);
                        }
                    }
                }
                None => {}
            }
        }
    }

    let mut processed_stamp = None;
    let bound_queue = queue.clone();
    for i in bound_queue.keys().sorted() {
        if *i > stamp.large {
            break;
        }
        let msgs = queue.get(i).unwrap();

        for m in msgs {
            outgoing.send(m.clone());
        }
        processed_stamp = Some(i);
        break;
    }

    match processed_stamp {
        Some(i) => {
            queue.remove(&i);
        }
        None => {}
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
) {
    while let Some(message) = client.receive_message(RENET_UNRELIABLE_CHANNEL_ID) {
        match bincode::deserialize::<UnreliableServerMessageBatch>(&message) {
            Ok(msg) => {
                events.send(IncomingRawUnreliableServerMessage { message: msg });
            }
            Err(_) => {
                warn!("Received an invalid message 0.");
            }
        }
    }
}
#[derive(Resource, Default)]
pub(crate) struct RawServerMessageQueue {
    pub reliable: Vec<(u64, ReliableServerMessageBatch)>,
}

pub(crate) fn step_incoming_reliable_server_messages(
    mut queue: ResMut<RawServerMessageQueue>,
    mut events: EventWriter<IncomingRawReliableServerMessage>,
) {
    let mut lowest_stamp = u64::MAX;
    for (s, _) in queue.reliable.iter() {
        if *s < lowest_stamp {
            lowest_stamp = *s;
        }
    }
    let mut remove_i = vec![];
    let mut i = 0;
    for (s, message) in queue.reliable.iter() {
        if *s == lowest_stamp {
            remove_i.push(i);
            events.send(IncomingRawReliableServerMessage {
                message: message.clone(),
            });
        }
        i += 1;
    }
    for i in remove_i.iter().rev() {
        queue.reliable.remove(*i);
    }
}

/// Deserializes incoming server messages and writes to event.

pub(crate) fn receive_incoming_reliable_server_messages(
    mut client: ResMut<RenetClient>,
    mut queue: ResMut<RawServerMessageQueue>,
    stamp: Res<TickRateStamp>,
) {
    while let Some(message) = client.receive_message(RENET_RELIABLE_ORDERED_ID) {
        match bincode::deserialize::<ReliableServerMessageBatch>(&message) {
            Ok(msg) => {
                //events.send(IncomingRawReliableServerMessage { message: msg });
                queue.reliable.push((stamp.calculate_large(msg.stamp), msg));
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
    }
    while let Some(message) = client.receive_message(RENET_RELIABLE_UNORDERED_ID) {
        match bincode::deserialize::<ReliableServerMessageBatch>(&message) {
            Ok(msg) => {
                //events.send(IncomingRawReliableServerMessage { message: msg });
                queue.reliable.push((stamp.calculate_large(msg.stamp), msg));
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
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
#[derive(Event)]
pub struct IncomingRawReliableServerMessage {
    pub message: ReliableServerMessageBatch,
}

/// Event to when received reliable message from server. Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
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

use crate::server::NetworkingServerMessage;
#[derive(Resource, Default)]
pub(crate) struct SyncClient((bool, u8));

pub(crate) fn sync_frequency(mut first: ResMut<SyncClient>) {
    first.0 .0 = true;
}
pub(crate) const NEW_SYNC_FREQUENCY: f32 = 0.1;
const TEST_MESSAGES_AMOUNT: u8 = 2;
// Waits this amount of ticks per sync message send.
const TEST_MESSAGES_FREQUENCY: f32 = 4.;

pub(crate) fn sync_test_client(
    mut first: ResMut<SyncClient>,
    mut net: EventWriter<OutgoingReliableClientMessage<NetworkingClientMessage>>,
    mut skip: Local<u8>,
    rate: Res<TickRate>,
) {
    if first.0 .0 {
        if *skip == u8::MAX {
            *skip = 0;
        } else {
            *skip += 1;
        }
        if *skip > (TEST_MESSAGES_FREQUENCY * (rate.fixed_rate as f32 / 60.)).round() as u8 {
            first.0 .1 += 1;

            if first.0 .1 > TEST_MESSAGES_AMOUNT {
                first.0 .0 = false;
                first.0 .1 = 0;
            } else {
                net.send(OutgoingReliableClientMessage {
                    message: NetworkingClientMessage::HeartBeat,
                });
            }
            *skip = 0;
        }
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
    HeartBeat,
    SyncConfirmation,
    LoadedGameWorld,
    StartSyncConfirmation,
}
