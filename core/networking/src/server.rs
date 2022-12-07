use bevy::{
    math::{Vec2, Vec3},
    prelude::{info, Component, Entity, Quat, Resource},
};
use serde::{Deserialize, Serialize};
use typename::TypeName;

use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, RenetServer, ServerAuthentication,
    ServerConfig,
};

/// The network port the server will listen use for connections.
#[cfg(feature = "server")]
pub const SERVER_PORT: u16 = 57713;

/// Network protocol ID.
#[cfg(any(feature = "server", feature = "client"))]
pub(crate) const PROTOCOL_ID: u64 = 7;

/// Start server and open and listen to port.
#[cfg(feature = "server")]
pub(crate) fn startup_server_listen_connections() -> RenetServer {
    let server_addr = (local_ipaddress::get().unwrap_or_default() + ":" + &SERVER_PORT.to_string())
        .parse()
        .unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();

    let channels_config = vec![
        ChannelConfig::Reliable(ReliableChannelConfig {
            packet_budget: 6000,
            max_message_size: 5900,
            ..Default::default()
        }),
        ChannelConfig::Unreliable(Default::default()),
        ChannelConfig::Chunk(Default::default()),
    ];

    let connection_config = RenetConnectionConfig {
        send_channels_config: channels_config.clone(),
        receive_channels_config: channels_config,
        ..Default::default()
    };

    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let renet_server =
        RenetServer::new(current_time, server_config, connection_config, socket).unwrap();

    info!("Listening to connections on [{}].", server_addr);

    renet_server
}

use bevy::prelude::EventReader;

use crate::messaging::{ReliableMessage, UnreliableMessage};

/// Obtain player souls, mwahahhaa. (=^.^=)
#[cfg(feature = "server")]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[cfg(feature = "server")]
pub enum GridMapLayer {
    Main,
    Details1,
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum NetworkingClientMessage {
    HeartBeat,
}

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum UnreliableServerMessage {
    TransformUpdate(u64, Vec3, Quat, Option<Vec3>, u64, u8),
    PositionUpdate(u64, Vec3, u64),
}

/// Variants for input console commands with values.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GodotVariantValues {
    Int(i64),
    String(String),
    Float(f32),
    Bool(bool),
}
/// Variant types for input console commands with values.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GodotVariant {
    Int,
    String,
    Float,
    Bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum UIInputAction {
    Pressed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum TextTreeBit {
    Final(Vec<String>),
    Bit(HashMap<String, TextTreeBit>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
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
#[cfg(feature = "server")]
pub struct HandleToEntity {
    pub map: HashMap<u64, Entity>,
    pub inv_map: HashMap<Entity, u64>,
}

/// The component for an entity controlled by a connected player.
#[derive(Component, Clone)]
#[cfg(feature = "server")]
pub struct ConnectedPlayer {
    pub handle: u64,
    pub authid: u16,
    pub rcon: bool,
    pub connected: bool,
}
#[cfg(feature = "server")]
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
#[cfg(any(feature = "server", feature = "client"))]
pub enum NetworkingChatServerMessage {
    ChatMessage(String),
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum GreetingClientServerMessage {
    Awoo,
}

use bevy::prelude::warn;

use crate::plugin::RENET_UNRELIABLE_CHANNEL_ID;
/// Serializes and sends the outgoing unreliable server messages.
#[cfg(any(feature = "server"))]
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
#[cfg(any(feature = "server"))]
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

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub struct IncomingReliableClientMessage<T: TypeName + Send + Sync + Serialize> {
    pub handle: u64,
    pub message: T,
}
///  Messages that you receive with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[cfg(feature = "server")]
pub struct IncomingUnreliableClientMessage<T: TypeName + Send + Sync + Serialize> {
    pub handle: u64,
    pub message: T,
}

/// Deserializes header of incoming client messages and writes to event.
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub struct OutgoingReliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: u64,
    pub message: T,
}

/// Event to send unreliable messages from server to client. Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[cfg(feature = "server")]
pub struct OutgoingUnreliableServerMessage<T: TypeName + Send + Sync + 'static> {
    pub handle: u64,
    pub message: T,
}
/// Event to when received reliable message from client.  Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_reliable_message].
#[cfg(feature = "server")]
pub(crate) struct IncomingRawReliableClientMessage {
    pub handle: u64,
    pub message: ReliableMessage,
}
/// Event to when received reliable message from client.  Messages that you use with this event must be initiated from a plugin builder with [crate::messaging::init_unreliable_message].
#[cfg(feature = "server")]
pub(crate) struct IncomingRawUnreliableClientMessage {
    pub handle: u64,
    pub message: UnreliableMessage,
}
