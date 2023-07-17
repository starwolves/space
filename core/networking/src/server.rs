use bevy::{
    math::{Vec2, Vec3},
    prelude::{info, Component, Entity, Event, Quat, Resource},
};
use serde::{Deserialize, Serialize};
use typename::TypeName;

use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, RenetServer,
};

/// The network port the server will listen use for connections.

pub const SERVER_PORT: u16 = 57713;

/// Network protocol ID.

pub(crate) const PROTOCOL_ID: u64 = 7;

pub(crate) const PRIV_KEY: [u8; 32] = *b"(=^.^=)(=^.^=)(=^.^=)(=^.^=)(=^.";

/// Start server and open and listen to port.

pub(crate) fn startup_server_listen_connections() -> (RenetServer, NetcodeServerTransport) {
    let public_addr = (local_ipaddress::get().unwrap_or_default() + ":" + &SERVER_PORT.to_string())
        .parse()
        .unwrap();
    let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();
    let renet_server = RenetServer::new(ConnectionConfig::default());

    let server_config = ServerConfig {
        max_clients: 128,
        protocol_id: PROTOCOL_ID,
        public_addr,
        authentication: ServerAuthentication::Secure {
            private_key: PRIV_KEY,
        },
    };
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    info!("Listening to connections on [{}].", public_addr);

    (renet_server, transport)
}

use bevy::prelude::EventReader;

use crate::messaging::{ReliableMessage, UnreliableMessage};

/// Obtain player souls, mwahahhaa. (=^.^=)

pub(crate) fn souls(
    mut server: EventReader<IncomingReliableClientMessage<NetworkingClientMessage>>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();
        match client_message {
            //                                          |
            // Where the souls of the players are       |
            //   while they're connected.               V
            NetworkingClientMessage::HeartBeat => { /* <3 */ }
        }
    }
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum NetworkingClientMessage {
    HeartBeat,
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum NetworkingServerMessage {
    Awoo,
}

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UnreliableServerMessage {
    TransformUpdate(u64, Vec3, Quat, Option<Vec3>, u64, u8),
    PositionUpdate(u64, Vec3, u64),
}

/// Variant types for input console commands with values.
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum ConsoleArgVariant {
    Int,
    String,
    Float,
    Bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum UIInputAction {
    Pressed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum TextTreeBit {
    Final(Vec<String>),
    Bit(HashMap<String, TextTreeBit>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum EntityUpdateData {
    Int(i64),
    UInt8(u8),
    String(String),
    StringVec(Vec<String>),
    Float(f32),
    Transform(Vec3, Quat, Vec3),
    Color(f32, f32, f32, f32),
    Bool(bool),
    Vec3(Vec3),
    Vec2(Vec2),
    AttachedItem(u64, Vec3, Quat, Vec3),
    WornItem(String, u64, String, Vec3, Quat, Vec3),
    WornItemNotAttached(String, u64, String),
}

/// A resource that links entities to their appropiate connection handles for connected players.
#[derive(Default, Resource)]

pub struct HandleToEntity {
    pub map: HashMap<u64, Entity>,
    pub inv_map: HashMap<Entity, u64>,
}

/// The component for an entity controlled by a connected player.
#[derive(Component, Clone)]

pub struct ConnectedPlayer {
    pub handle: u64,
    pub authid: u16,
    pub rcon: bool,
    pub connected: bool,
}

impl Default for ConnectedPlayer {
    fn default() -> Self {
        Self {
            handle: 0,
            authid: 0,
            rcon: false,
            connected: true,
        }
    }
}

/// Gets serialized and sent over the net, this is the server message.
/// This should be inside core/chat/ but this causes cyclic dependency for the time being.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum NetworkingChatServerMessage {
    ChatMessage(String),
}

use bevy::prelude::warn;

use crate::plugin::RENET_UNRELIABLE_CHANNEL_ID;
/// Serializes and sends the outgoing unreliable server messages.
pub(crate) fn send_outgoing_unreliable_server_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingUnreliableServerMessage<T>>,
    mut server: ResMut<RenetServer>,
    typenames: Res<Typenames>,
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
        }) {
            Ok(bits) => {
                server.send_message(message.handle, RENET_UNRELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize unreliable message.");
                continue;
            }
        }
    }
}

use bevy::prelude::{Res, ResMut};

use crate::messaging::Typenames;
/// Serializes and sends the outgoing reliable server messages.
pub(crate) fn send_outgoing_reliable_server_messages<T: TypeName + Send + Sync + Serialize>(
    mut events: EventReader<OutgoingReliableServerMessage<T>>,
    mut server: ResMut<RenetServer>,
    typenames: Res<Typenames>,
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
                warn!(
                    "Couldnt find server reliable type {}",
                    message.message.type_name_of()
                );
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

        match bincode::serialize(&ReliableMessage {
            serialized: bin,
            typename_net: *net,
        }) {
            Ok(bits) => {
                server.send_message(message.handle, RENET_RELIABLE_CHANNEL_ID, bits);
            }
            Err(_) => {
                warn!("Failed to serialize reliable message.");
                continue;
            }
        }
    }
}
use crate::client::get_unreliable_message;
use bevy::prelude::EventWriter;

pub(crate) fn deserialize_incoming_unreliable_client_message<
    T: TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawUnreliableClientMessage>,
    mut outgoing: EventWriter<IncomingUnreliableClientMessage<T>>,
    typenames: Res<Typenames>,
) {
    for event in incoming_raw.iter() {
        match get_unreliable_message::<T>(
            &typenames,
            event.message.typename_net,
            &event.message.serialized,
        ) {
            Some(data) => {
                outgoing.send(IncomingUnreliableClientMessage {
                    message: data,
                    handle: event.handle,
                });
            }
            None => {}
        }
    }
}
use crate::messaging::get_reliable_message;

pub(crate) fn deserialize_incoming_reliable_client_message<
    T: TypeName + Send + Sync + Serialize + for<'a> Deserialize<'a> + 'static,
>(
    mut incoming_raw: EventReader<IncomingRawReliableClientMessage>,
    mut outgoing: EventWriter<IncomingReliableClientMessage<T>>,
    typenames: Res<Typenames>,
) {
    for event in incoming_raw.iter() {
        match get_reliable_message::<T>(
            &typenames,
            event.message.typename_net,
            &event.message.serialized,
        ) {
            Some(data) => {
                outgoing.send(IncomingReliableClientMessage {
                    message: data,
                    handle: event.handle,
                });
            }
            None => {}
        }
    }
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event)]
pub struct IncomingReliableClientMessage<T: TypeName + Send + Sync + Serialize> {
    pub handle: u64,
    pub message: T,
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
pub struct IncomingUnreliableClientMessage<T: TypeName + Send + Sync + Serialize> {
    pub handle: u64,
    pub message: T,
}

/// Deserializes header of incoming client messages and writes to event.

pub(crate) fn receive_incoming_unreliable_client_messages(
    mut events: EventWriter<IncomingRawUnreliableClientMessage>,
    mut server: ResMut<RenetServer>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_UNRELIABLE_CHANNEL_ID) {
            match bincode::deserialize::<UnreliableMessage>(&message) {
                Ok(msg) => {
                    events.send(IncomingRawUnreliableClientMessage {
                        message: msg,
                        handle,
                    });
                }
                Err(_) => {
                    warn!("Received an invalid message.");
                }
            }
        }
    }
}
use crate::plugin::RENET_RELIABLE_CHANNEL_ID;

/// Deserializes header of incoming client messages and writes to event.

pub(crate) fn receive_incoming_reliable_client_messages(
    mut events: EventWriter<IncomingRawReliableClientMessage>,
    mut server: ResMut<RenetServer>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            match bincode::deserialize::<ReliableMessage>(&message) {
                Ok(msg) => {
                    events.send(IncomingRawReliableClientMessage {
                        message: msg,
                        handle,
                    });
                }
                Err(_) => {
                    warn!("Received an invalid message.");
                }
            }
        }
    }
}

/// Event to send reliable messages from server to client. Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event)]
pub struct OutgoingReliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: u64,
    pub message: T,
}

/// Event to send unreliable messages from server to client. Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
pub struct OutgoingUnreliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: u64,
    pub message: T,
}
/// Event to when received reliable message from client.  Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[derive(Event)]
pub(crate) struct IncomingRawReliableClientMessage {
    pub handle: u64,
    pub message: ReliableMessage,
}
/// Event to when received reliable message from client.  Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[derive(Event)]
pub(crate) struct IncomingRawUnreliableClientMessage {
    pub handle: u64,
    pub message: UnreliableMessage,
}
