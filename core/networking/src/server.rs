use bevy::{
    math::{Vec2, Vec3},
    prelude::{info, Component, Entity, Quat, ResMut, Resource},
};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, RenetServer, ServerAuthentication,
    ServerConfig,
};

use super::plugin::RENET_RELIABLE_CHANNEL_ID;

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

/// Obtain player souls, mwahahhaa. (=^.^=)
#[cfg(feature = "server")]
pub(crate) fn souls(mut net: ResMut<RenetServer>) {
    for handle in net.clients_id().into_iter() {
        while let Some(message) = net.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<NetworkingMessage, _> =
                bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    continue;
                }
            }
            match client_message {
                //                                        |
                // Where the souls of the players are     |
                //   while they're connected.             V
                NetworkingMessage::HeartBeat => { /* <3 */ }
            }
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum NetworkingMessage {
    HeartBeat,
}

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum NetworkingChatServerMessage {
    ChatMessage(String),
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum NetworkingClientServerMessage {
    Awoo,
}
