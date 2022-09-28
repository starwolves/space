use std::collections::HashMap;

use bevy::{
    math::{Quat, Vec2, Vec3},
    prelude::Entity,
};
use serde::{Deserialize, Serialize};

use crate::{
    combat::ProjectileData,
    console_commands::{ConsoleCommandVariant, ConsoleCommandVariantValues},
    entity_updates::EntityUpdateData,
    gridmap::GridMapLayer,
    ui::{UIInputAction, UIInputNodeClass},
    world_environment::WorldEnvironment,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextTreeBit {
    Final(Vec<String>),
    Bit(HashMap<String, TextTreeBit>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetAction {
    pub id: String,
    pub text: String,
    pub tab_list_priority: u8,
    pub item_name: String,
    pub entity_option: Option<u64>,
    pub belonging_entity: Option<u64>,
    pub cell_option: Option<(GridMapLayer, i16, i16, i16)>,
}

pub enum NetMessageType {
    Reliable(ReliableServerMessage),
    Unreliable(UnreliableServerMessage),
}

/// Input chat message event.
pub struct InputChatMessage {
    pub entity: Entity,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityWorldType {
    Main,
    HealthUI,
}

pub trait PendingMessage {
    fn get_message(&self) -> PendingNetworkMessage;
}

pub struct PendingNetworkMessage {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for PendingNetworkMessage {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Gets serialized and sent over the net, this is the client message.
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
    ExamineMap(GridMapLayer, i16, i16, i16),
    TabDataEntity(u64),
    TabDataMap(GridMapLayer, i16, i16, i16),
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

/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnreliableServerMessage {
    TransformUpdate(u64, Vec3, Quat, Option<Vec3>, u64, u8),
    PositionUpdate(u64, Vec3, u64),
}
/// This message gets sent at high intervals.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnreliableClientMessage {
    MouseDirectionUpdate(f32, u64),
    MapViewRange(f32),
    MapOverlayMouseHoverCell(i16, i16),
}
