use bevy::{
    math::{Vec2, Vec3},
    prelude::{info, warn, Entity, EventReader, EventWriter, Quat, ResMut},
};
use chat_api::core::ASTRIX;
use math::grid::Vec3Int;
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
        ChannelConfig::Block(Default::default()),
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

/// Client input drop current item event.
#[cfg(feature = "server")]
pub struct InputDropCurrentItem {
    pub pickuper_entity: Entity,
    /// Drop item on position, for placeable item surfaces.
    pub input_position_option: Option<Vec3>,
}

/// Client input throw item event.
#[cfg(feature = "server")]
pub struct InputThrowItem {
    pub entity: Entity,
    pub position: Vec3,
    pub angle: f32,
}

/// Client input switch hands event.
#[cfg(feature = "server")]
pub struct InputSwitchHands {
    pub entity: Entity,
}

/// Client input take off item event.
#[cfg(feature = "server")]
pub struct InputTakeOffItem {
    pub entity: Entity,
    pub slot_name: String,
}

/// Client input use world item event.
#[cfg(feature = "server")]
pub struct InputUseWorldItem {
    pub using_entity: Entity,
    pub used_entity: Entity,
}

/// Client input wear item event.
#[cfg(feature = "server")]
pub struct InputWearItem {
    pub wearer_entity: Entity,
    pub worn_entity_bits: u64,
    pub wear_slot: String,
}
/// Client input user name event.
#[cfg(feature = "server")]
pub struct InputAccountName {
    pub entity: Entity,
    pub input_name: String,
}
/// Client input list actions entity event.
#[derive(Clone)]
#[cfg(feature = "server")]
pub struct InputListActionsEntity {
    pub requested_by_entity: Entity,
    /// Targetted entity.
    pub targetted_entity: Entity,
    /// Whether UI should be displayed to the requested by entity.
    pub with_ui: bool,
}

/// Client initiates execution of an action event.
#[cfg(feature = "server")]
pub struct InputAction {
    /// Action ID.
    pub fired_action_id: String,
    pub action_taker: Entity,
    pub target_entity_option: Option<Entity>,
    pub target_cell_option: Option<(GridMapLayer, Vec3Int)>,
}

/// Client input toggle combat mode event.
#[cfg(feature = "server")]
pub struct InputToggleCombatMode {
    pub entity: Entity,
}

/// Client input toggle auto move event.
#[cfg(feature = "server")]
pub struct InputToggleAutoMove {
    pub entity: Entity,
}

/// Client input attack entity event.
#[cfg(feature = "server")]
pub struct InputAttackEntity {
    pub entity: Entity,
    pub target_entity_bits: u64,
}

/// Client input alt item attack event.
#[cfg(feature = "server")]
pub struct InputAltItemAttack {
    pub entity: Entity,
}

/// Client input mouse action event.
#[cfg(feature = "server")]
pub struct InputMouseAction {
    pub entity: Entity,
    pub pressed: bool,
}
/// Client input select body part event.
#[cfg(feature = "server")]
pub struct InputSelectBodyPart {
    pub entity: Entity,
    pub body_part: String,
}
/// Client input movement event.
#[cfg(feature = "server")]
pub struct InputMovementInput {
    pub player_entity: Entity,
    pub vector: Vec2,
}

/// Client text tree input selection event.
#[cfg(feature = "server")]
pub struct TextTreeInputSelection {
    /// Handle of the submitter of the selection.
    pub handle: u64,
    /// Menu ID.
    pub menu_id: String,
    /// The selection on the menu.
    pub menu_selection: String,
    /// The action ID.
    pub action_id: String,
    /// The entity submitting the selection.
    pub belonging_entity: Option<u64>,
}

/// Client input sprinting event.
#[cfg(feature = "server")]
pub struct InputSprinting {
    pub entity: Entity,
    pub is_sprinting: bool,
}
/// Client input scene ready event.
#[cfg(feature = "server")]
pub struct InputSceneReady {
    pub handle: u64,
    pub scene_id: String,
}
/// Client input build graphics event.
#[cfg(feature = "server")]
pub struct InputBuildGraphics {
    pub handle: u64,
}

/// Client input mouse direction update event.
#[cfg(feature = "server")]
pub struct InputMouseDirectionUpdate {
    pub entity: Entity,
    pub direction: f32,
    pub time_stamp: u64,
}
/// Client input construction options selection event.
#[cfg(feature = "server")]
pub struct InputConstructionOptionsSelection {
    pub handle_option: Option<u64>,
    pub menu_selection: String,
    // Entity has been validated.
    pub entity: Entity,
}
/// Input examine entity event.
#[derive(Clone)]
#[cfg(feature = "server")]
pub struct InputExamineEntity {
    pub handle: u64,
    pub examine_entity: Entity,
    pub entity: Entity,
    /// Examine message that is being built and returned to the client.
    pub message: String,
}
#[cfg(feature = "server")]
impl Default for InputExamineEntity {
    fn default() -> Self {
        Self {
            handle: 0,
            examine_entity: Entity::from_bits(0),
            entity: Entity::from_bits(0),
            message: ASTRIX.to_string(),
        }
    }
}

/// Event as client input , interaction with UI.
#[cfg(feature = "server")]
pub struct InputUIInput {
    /// Handle of the connection that input this.
    pub handle: u64,
    /// The Godot node class of the input element.
    pub node_class: UIInputNodeClass,
    /// The action ID.
    pub action: UIInputAction,
    /// The Godot node name of the input element.
    pub node_name: String,
    /// The UI this input was submitted from.
    pub ui_type: String,
}

/// Client input submitting text event.
#[cfg(feature = "server")]
pub struct InputUIInputTransmitText {
    /// Handle of the connection that input this.
    pub handle: u64,
    /// The UI this input was submitted from.
    pub ui_type: String,
    /// The Godot node path of the input element.
    pub node_path: String,
    /// The input text from the client.
    pub input_text: String,
}
/// Examine map message event.
#[derive(Clone)]
#[cfg(feature = "server")]
pub struct InputExamineMap {
    pub handle: u64,
    pub entity: Entity,
    pub gridmap_type: GridMapLayer,
    pub gridmap_cell_id: Vec3Int,
    /// Map examine message being built and sent back to the player.
    pub message: String,
}
#[cfg(feature = "server")]
impl Default for InputExamineMap {
    fn default() -> Self {
        Self {
            handle: 0,
            entity: Entity::from_bits(0),
            gridmap_type: GridMapLayer::Main,
            gridmap_cell_id: Vec3Int::default(),
            message: ASTRIX.to_string(),
        }
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
    UIRequestInput(String, String),
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

/// Input chat message event.
#[cfg(feature = "server")]
pub struct InputChatMessage {
    pub entity: Entity,
    pub message: String,
}

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetSendEntityUpdates {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetHealth {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetUnloadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetLoadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
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
