use bevy::{
    math::{Vec2, Vec3},
    prelude::{info, warn, Component, Entity, EventReader, EventWriter, Quat, ResMut, Resource},
};
use networking_macros::NetMessage;
use serde::{Deserialize, Serialize};
use world_environment::environment::WorldEnvironment;

use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, RenetServer, ServerAuthentication,
    ServerConfig, NETCODE_KEY_BYTES,
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
pub(crate) fn startup_server_listen_connections(
    encryption_key: [u8; NETCODE_KEY_BYTES],
) -> RenetServer {
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

    let server_config = ServerConfig::new(
        64,
        PROTOCOL_ID,
        server_addr,
        ServerAuthentication::Secure {
            private_key: encryption_key,
        },
    );
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
            let client_message_result: Result<ReliableClientMessage, _> =
                bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    warn!("Received invalid client message.");
                    continue;
                }
            }
            match client_message {
                //                                        |
                // Where the souls of the players are     |
                //   while they're connected.             V
                ReliableClientMessage::HeartBeat => { /* <3 */ }
                _ => (),
            }
        }
    }
}

/// Net message handler.
#[cfg(feature = "server")]
pub fn net_system<T: std::marker::Send + std::marker::Sync + PendingMessage + 'static>(
    mut net1: EventReader<T>,
    mut pending_net: EventWriter<PendingNetworkMessage>,
) {
    for new_event in net1.iter() {
        let message = new_event.get_message();

        pending_net.send(PendingNetworkMessage {
            handle: message.handle,
            message: message.message,
        });
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[cfg(feature = "server")]
pub enum GridMapLayer {
    Main,
    Details1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(feature = "server")]
pub struct NetAction {
    pub id: String,
    pub text: String,
    pub tab_list_priority: u8,
    pub item_name: String,
    pub entity_option: Option<u64>,
    pub belonging_entity: Option<u64>,
    pub cell_option: Option<(GridMapLayer, i16, i16, i16)>,
}

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ReliableClientMessage {
    Awoo,
    HeartBeat,
    UIInput(UIInputNodeClass, UIInputAction, String, String),
    SceneReady(String),
    UIInputTransmitData(String, String, String),
    MovementInput(Vec2),
    SprintInput(bool),
    BuildGraphics,
    InputChatMessage(String),
    ExamineEntity(u64),
    ExamineMap(GridMapLayer, i16, i16, i16),
    TabDataEntity(u64),
    TabDataMap(GridMapLayer, i16, i16, i16),
    UseWorldItem(u64),
    DropCurrentItem(Option<Vec3>),
    SwitchHands,
    WearItem(u64, String),
    TakeOffItem(String),
    ConsoleCommand(String, Vec<GodotVariantValues>),
    ToggleCombatModeInput,
    InputMouseAction(bool),
    SelectBodyPart(String),
    ToggleAutoMove,
    AccountName(String),
    AttackEntity(u64),
    AltItemAttack,
    ThrowItem(Vec3, f32),
    AttackCell(i16, i16, i16),
    TabPressed(
        String,
        Option<u64>,
        Option<(GridMapLayer, i16, i16, i16)>,
        Option<u64>,
    ),
    TextTreeInput(Option<u64>, String, String, String),
    MapChangeDisplayMode(String),
    MapRequestDisplayModes,
    MapCameraPosition(Vec2),
}
/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ReliableServerMessage {
    EntityUpdate(
        u64,
        HashMap<String, HashMap<String, EntityUpdateData>>,
        bool,
        EntityWorldType,
    ),
    ConfigMessage(ServerConfigMessage),
    UIRequestInput(String),
    LoadEntity(
        String,
        String,
        HashMap<String, HashMap<String, EntityUpdateData>>,
        u64,
        bool,
        String,
        String,
        bool,
    ),
    UnloadEntity(u64, bool),
    ChatMessage(String),
    PickedUpItem(String, u64, String),
    DropItem(String),
    SwitchHands,
    EquippedWornItem(String, u64, String),
    ConsoleWriteLine(String),
    PlaySound(String, f32, f32, Option<Vec3>),
    FireProjectile(ProjectileData),
    TabData(Vec<NetAction>),
    TextTreeSelection(
        Option<u64>,
        String,
        String,
        String,
        HashMap<String, TextTreeBit>,
    ),
    RemoveCell(i16, i16, i16, GridMapLayer),
    AddCell(i16, i16, i16, i64, i64, GridMapLayer),
    MapSendDisplayModes(Vec<(String, String)>),
    MapOverlayUpdate(Vec<(i16, i16, i16)>),
    MapOverlayHoverData(String),
    UIAddNotice(String),
    UIRemoveNotice(String),
    MapDefaultAddition(i16, i16, i16),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ServerConfigMessage {
    Awoo,
    WorldEnvironment(WorldEnvironment),
    ServerTime,
    ConnectedPlayers(u16),
    TickRate(u8),
    EntityId(u64),
    BlackCellID(i64, i64),
    OrderedCellsMain(Vec<String>),
    OrderedCellsDetails1(Vec<String>),
    ChangeScene(bool, String),
    ServerEntityId(u64),
    RepeatingSFX(String, Vec<String>),
    FinishedInitialization,
    ConsoleCommands(Vec<(String, String, Vec<(String, GodotVariant)>)>),
    TalkSpaces(Vec<(String, String)>),
    PlaceableItemsSurfaces(Vec<i64>),
    NonBlockingCells(Vec<i64>),
}

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum UnreliableServerMessage {
    TransformUpdate(u64, Vec3, Quat, Option<Vec3>, u64, u8),
    PositionUpdate(u64, Vec3, u64),
}
/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum UnreliableClientMessage {
    MouseDirectionUpdate(f32, u64),
    MapViewRange(f32),
    MapOverlayMouseHoverCell(i16, i16),
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
pub enum UIInputNodeClass {
    Button,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum UIInputAction {
    Pressed,
}

/// Contains information about the projectile and its visual graphics.
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ProjectileData {
    Laser((f32, f32, f32, f32), f32, f32, Vec3, Vec3),
    Ballistic,
}

#[cfg(feature = "server")]
pub trait PendingMessage {
    fn get_message(&self) -> PendingNetworkMessage;
}
/// Only NetMessage that shouldnt be pub(crate)
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct PendingNetworkMessage {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

#[cfg(any(feature = "server", feature = "client"))]
pub enum NetMessageType {
    Reliable(ReliableServerMessage),
    Unreliable(UnreliableServerMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum EntityWorldType {
    Main,
    HealthUI,
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

use bevy::prelude::{Query, Res};
/// Finalize netcode messages system.
#[cfg(feature = "server")]
pub(crate) fn process_finalize_net(
    mut pending_network_message: EventReader<PendingNetworkMessage>,
    connected_players: Query<&ConnectedPlayer>,
    mut net: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for p in pending_network_message.iter() {
        finalize_send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: p.handle,
                message: p.message.clone(),
            },
        );
    }
}
#[derive(NetMessage)]
pub(crate) struct NetEvent {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

/// Finalize sending netcode messages to clients as a function.
#[cfg(feature = "server")]
pub(crate) fn finalize_send_net(
    net: &mut ResMut<RenetServer>,
    connected_players: &Query<&ConnectedPlayer>,
    handle_to_entity: &Res<HandleToEntity>,
    new_event: &NetEvent,
) {
    use bincode::serialize;

    let mut connected = false;

    match handle_to_entity.map.get(&new_event.handle) {
        Some(r) => match connected_players.get(*r) {
            Ok(rr) => {
                if rr.connected {
                    connected = true;
                }
            }
            Err(_rr) => {
                connected = true;
            }
        },
        None => {
            warn!("Couldnt find handle entity!");
            return;
        }
    }
    if !connected {
        return;
    }
    net.send_message(
        new_event.handle,
        RENET_RELIABLE_CHANNEL_ID,
        serialize::<ReliableServerMessage>(&new_event.message).unwrap(),
    );
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
