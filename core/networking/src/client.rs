use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::{
    prelude::{error, info, Event, Resource},
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_renet::renet::{
    transport::{ClientAuthentication, ConnectToken, NetcodeClientTransport},
    ConnectionConfig, DefaultChannel, RenetClient,
};
use futures_lite::future;
use token::parse::Token;

use crate::{
    messaging::ReliableClientMessage, plugin::RENET_RELIABLE_ORDERED_ID, server::PROTOCOL_ID,
    sync::TickRateStamp,
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
use bevy::prelude::warn;
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
    for _ in events.iter() {
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
    for _ in event.iter() {
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

/// System run run_if with iyes_loopless

pub fn connected(connection: Res<Connection>) -> bool {
    matches!(connection.status, ConnectionStatus::Connected)
}
/// System run run_if with iyes_loopless. The earliest server messages (for setup_ui, boarding etc.)
/// come in while in the connecting stage.

pub fn is_client_connected(connection: Res<Connection>) -> bool {
    matches!(connection.status, ConnectionStatus::Connecting)
        || matches!(connection.status, ConnectionStatus::Connected)
}
use crate::messaging::ReliableServerMessage;
use crate::messaging::Typenames;
use crate::plugin::RENET_UNRELIABLE_CHANNEL_ID;

use serde::Serialize;
use typename::TypeName;

#[derive(Resource, Default)]
pub struct OutgoingBuffer {
    pub reliable: Vec<Vec<u8>>,
    pub unreliable: Vec<Vec<u8>>,
}

pub(crate) fn step_buffer(mut res: ResMut<OutgoingBuffer>, mut client: ResMut<RenetClient>) {
    for message in res.reliable.iter() {
        client.send_message(RENET_RELIABLE_ORDERED_ID, message.clone());
    }
    for message in res.unreliable.iter() {
        client.send_message(RENET_UNRELIABLE_CHANNEL_ID, message.clone())
    }

    res.reliable.clear();
    res.unreliable.clear();
}

/// Serializes and sends the outgoing reliable client messages.
pub(crate) fn send_outgoing_reliable_client_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingReliableClientMessage<T>>,
    mut client: ResMut<OutgoingBuffer>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    for message in events.iter() {
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
        match bincode::serialize(&ReliableClientMessage {
            serialized: bin,
            typename_net: *net,
            stamp: stamp.stamp,
        }) {
            Ok(bits) => {
                client.reliable.push(bits);
            }
            Err(_) => {
                warn!("Failed to serialize unreliable message.");
                continue;
            }
        }
    }
}
use crate::messaging::UnreliableMessage;

/// Serializes and sends the outgoing unreliable client messages.
pub(crate) fn send_outgoing_unreliable_client_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingUnreliableClientMessage<T>>,
    mut client: ResMut<OutgoingBuffer>,
    typenames: Res<Typenames>,
    stamp: Res<TickRateStamp>,
) {
    for message in events.iter() {
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

        match bincode::serialize(&UnreliableMessage {
            serialized: bin,
            typename_net: *net,
            stamp: stamp.stamp,
        }) {
            Ok(bits) => {
                client.unreliable.push(bits);
            }
            Err(_) => {
                warn!("Failed to serialize unreliable message.");
                continue;
            }
        }
    }
}
use serde::Deserialize;

pub(crate) fn deserialize_incoming_unreliable_server_message<
    T: TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawUnreliableServerMessage>,
    mut outgoing: EventWriter<IncomingUnreliableServerMessage<T>>,
    typenames: Res<Typenames>,
) {
    for event in incoming_raw.iter() {
        match get_unreliable_message::<T>(
            &typenames,
            event.message.typename_net,
            &event.message.serialized,
        ) {
            Some(data) => {
                outgoing.send(IncomingUnreliableServerMessage { message: data });
            }
            None => {}
        }
    }
}
use crate::messaging::get_reliable_message;

pub(crate) fn deserialize_incoming_reliable_server_message<
    T: TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawReliableServerMessage>,
    mut outgoing: EventWriter<IncomingReliableServerMessage<T>>,
    typenames: Res<Typenames>,
) {
    for event in incoming_raw.iter() {
        match get_reliable_message::<T>(
            &typenames,
            event.message.typename_net,
            &event.message.serialized,
        ) {
            Some(data) => {
                outgoing.send(IncomingReliableServerMessage { message: data });
            }
            None => {}
        }
    }
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event)]
pub struct IncomingReliableServerMessage<T: TypeName + Send + Sync + Serialize> {
    pub message: T,
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
pub struct IncomingUnreliableServerMessage<T: TypeName + Send + Sync + Serialize> {
    pub message: T,
}

/// Dezerializes incoming server messages and writes to event.

pub(crate) fn receive_incoming_unreliable_server_messages(
    mut events: EventWriter<IncomingRawUnreliableServerMessage>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(RENET_UNRELIABLE_CHANNEL_ID) {
        match bincode::deserialize::<UnreliableMessage>(&message) {
            Ok(msg) => {
                events.send(IncomingRawUnreliableServerMessage { message: msg });
            }
            Err(_) => {
                warn!("Received an invalid message.");
            }
        }
    }
}

/// Deserializes incoming server messages and writes to event.

pub(crate) fn receive_incoming_reliable_server_messages(
    mut events: EventWriter<IncomingRawReliableServerMessage>,
    mut client: ResMut<RenetClient>,
) {
    while let Some(message) = client.receive_message(RENET_RELIABLE_ORDERED_ID) {
        match bincode::deserialize::<ReliableServerMessage>(&message) {
            Ok(msg) => {
                events.send(IncomingRawReliableServerMessage { message: msg });
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
pub(crate) struct IncomingRawReliableServerMessage {
    pub message: ReliableServerMessage,
}

/// Event to when received reliable message from server. Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
pub(crate) struct IncomingRawUnreliableServerMessage {
    pub message: UnreliableMessage,
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

/// Confirms connection with server.

pub(crate) fn confirm_connection(
    mut client1: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut connected_state: ResMut<Connection>,
) {
    for message in client1.iter() {
        let player_message = message.message.clone();
        match player_message {
            NetworkingServerMessage::Awoo(_) => {
                connected_state.status = ConnectionStatus::Connected;
                info!("Connected.");
            }
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
