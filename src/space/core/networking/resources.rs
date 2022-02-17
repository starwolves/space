use std::{collections::HashMap, time::Duration};

use bevy::{
    math::{Quat, Vec2, Vec3},
    prelude::Color,
};
use bevy_networking_turbulence::{
    MessageChannelMode, MessageChannelSettings, ReliableChannelSettings,
};
use serde::{Deserialize, Serialize};

use crate::space::core::world_environment::resources::WorldEnvironment;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum GridMapType {
    Main,
    Details1,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    ExamineMap(GridMapType, i16, i16, i16),
    TabDataEntity(u64),
    TabDataMap(GridMapType, i16, i16, i16),
    UseWorldItem(u64),
    DropCurrentItem(Option<Vec3>),
    SwitchHands,
    WearItem(u64, String),
    TakeOffItem(String),
    ConsoleCommand(String, Vec<ConsoleCommandVariantValues>),
    ToggleCombatModeInput,
    InputMouseAction(bool),
    SelectBodyPart(String),
    ToggleAutoMove,
    UserName(String),
    AttackEntity(u64),
    AltItemAttack,
    ThrowItem(Vec3, f32),
    AttackCell(i16, i16, i16),
    TabPressed(
        String,
        Option<u64>,
        Option<(GridMapType, i16, i16, i16)>,
        Option<u64>,
    ),
    TextTreeInput(Option<u64>, String, String, String),
    MapChangeDisplayMode(String),
    MapRequestDisplayModes,
    MapCameraPosition(Vec2),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsoleCommandVariantValues {
    Int(i64),
    String(String),
    Float(f32),
    Bool(bool),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UIInputNodeClass {
    Button,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UIInputAction {
    Pressed,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NetProjectileType {
    Laser(Color, f32, f32, Vec3, Vec3),
    Ballistic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    FireProjectile(NetProjectileType),
    TabData(Vec<NetTabAction>),
    TextTreeSelection(
        Option<u64>,
        String,
        String,
        String,
        HashMap<String, TextTreeBit>,
    ),
    RemoveCell(i16, i16, i16, GridMapType),
    AddCell(i16, i16, i16, i64, i64, GridMapType),
    MapSendDisplayModes(Vec<(String, String)>),
    MapOverlayUpdate(Vec<(i16, i16, i16)>),
    MapOverlayHoverData(String),
    UIAddNotice(String),
    UIRemoveNotice(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextTreeBit {
    Final(Vec<String>),
    Bit(HashMap<String, TextTreeBit>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetTabAction {
    pub id: String,
    pub text: String,
    pub tab_list_priority: u8,
    pub item_name: String,
    pub entity_option: Option<u64>,
    pub belonging_entity: Option<u64>,
    pub cell_option: Option<(GridMapType, i16, i16, i16)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityWorldType {
    Main,
    HealthUI,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsoleCommandVariant {
    Int,
    String,
    Float,
    Bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityUpdateData {
    Int(i64),
    UInt8(u8),
    String(String),
    StringVec(Vec<String>),
    Float(f32),
    Transform(Vec3, Quat, Vec3),
    Color(Color),
    Bool(bool),
    Vec3(Vec3),
    Vec2(Vec2),
    AttachedItem(u64, Vec3, Quat, Vec3),
    WornItem(String, u64, String, Vec3, Quat, Vec3),
    WornItemNotAttached(String, u64, String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    ConsoleCommands(Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)>),
    TalkSpaces(Vec<(String, String)>),
    PlaceableItemsSurfaces(Vec<i64>),
    NonBlockingCells(Vec<i64>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnreliableServerMessage {
    TransformUpdate(u64, Vec3, Quat, Option<Vec3>, u64, u8),
    PositionUpdate(u64, Vec3, u64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnreliableClientMessage {
    MouseDirectionUpdate(f32, u64),
    MapViewRange(f32),
    MapOverlayMouseHoverCell(i16, i16),
}

pub enum NetMessageType {
    Reliable(ReliableServerMessage),
    Unreliable(UnreliableServerMessage),
}

pub const SERVER_MESSAGE_RELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 163840,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 32765,
    },
    message_buffer_size: 1024,
    packet_buffer_size: 1024,
};

pub const CLIENT_MESSAGE_RELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 163840,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 64,
    packet_buffer_size: 64,
};

pub const SERVER_MESSAGE_UNRELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 2,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 256,
    packet_buffer_size: 256,
};

pub const CLIENT_MESSAGE_UNRELIABLE: MessageChannelSettings = MessageChannelSettings {
    channel: 3,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 64,
    packet_buffer_size: 64,
};

pub const SERVER_PORT: u16 = 57713;
