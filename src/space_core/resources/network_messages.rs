use std::collections::HashMap;

use bevy::{math::{Quat, Vec2, Vec3}, prelude::Color};
use serde::{Serialize, Deserialize};

use crate::space_core::{resources::world_environments::WorldEnvironment};


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum GridMapType {
    Main,
    Details1
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReliableClientMessage {
    Awoo,
    HeartBeat,
    UIInput(UIInputNodeClass,UIInputAction,String,String),
    SceneReady(String),
    UIInputTransmitData(String, String, String),
    MovementInput(Vec2),
    SprintInput(bool),
    BuildGraphics,
    InputChatMessage(String),
    ExamineEntity(u64),
    ExamineMap(GridMapType, i16,i16,i16),
    UseWorldItem(u64),
    DropCurrentItem,
    SwitchHands,
    WearItem(u64, String),
    TakeOffItem(String),
    ConsoleCommand(String, Vec<ConsoleCommandVariantValues>),
    ToggleCombatModeInput,
    InputMouseAction(bool),
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
    Button
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UIInputAction {
    Pressed
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReliableServerMessage {
    EntityUpdate(u64, HashMap<String, HashMap<String, EntityUpdateData>>, bool),
    ConfigMessage(ServerConfigMessage),
    UIRequestInput(String, String),
    LoadEntity(String, String, HashMap<String, HashMap<String, EntityUpdateData>>, u64, bool, String, String, bool),
    UnloadEntity(u64, bool),
    ChatMessage(String),
    PickedUpItem(String, u64, String),
    DropItem(String),
    SwitchHands,
    EquippedWornItem(String, u64, String),
    ConsoleWriteLine(String),
    PlaySound(String, f32, f32),
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
    Transform(Vec3,Quat,Vec3),
    Color(Color),
    Bool(bool),
    Vec3(Vec3),
    Vec2(Vec2),
    AttachedItem(u64, Vec3,Quat,Vec3),
    WornItem(String, u64, String, Vec3,Quat,Vec3),
    WornItemNotAttached(String, u64, String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerConfigMessage {
    Awoo,
    WorldEnvironment(WorldEnvironment),
    TickRate(u8),
    EntityId(u64),
    BlackCellID(i64, i64),
    OrderedCellsMain(Vec<String>),
    OrderedCellsDetails1(Vec<String>),
    ChangeScene(bool, String),
    ServerEntityId(u64),
    RepeatingSFX(String, Vec<String>),
    FinishedInitialization,
    ConsoleCommands(Vec<(String,String, Vec<(String, ConsoleCommandVariant)>)>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnreliableServerMessage {
    TransformUpdate(u64, Vec3, Quat, Vec3, u64, u8),
    PositionUpdate(u64, Vec3, u64)
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnreliableClientMessage {
    MouseDirectionUpdate(f32, u64),
}
