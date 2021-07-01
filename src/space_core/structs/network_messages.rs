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
    UIInput(UIInputNodeClass,UIInputAction,String,String),
    SceneReady(String),
    UIInputTransmitData(String, String, String),
    MovementInput(Vec2),
    SprintInput(bool),
    BuildGraphics,
    InputChatMessage(String),
    ExamineEntity(u32),
    ExamineMap(GridMapType, i16,i16,i16)
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
    EntityUpdate(u32, HashMap<String, HashMap<String, EntityUpdateData>>),
    ConfigMessage(ServerConfigMessage),
    UIRequestInput(String, String),
    LoadEntity(String, String, HashMap<String, HashMap<String, EntityUpdateData>>, u32, bool, String, String, bool),
    UnloadEntity(u32, bool),
    ChatMessage(String)
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
    Vec3(Vec3)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerConfigMessage {
    Awoo,
    WorldEnvironment(WorldEnvironment),
    TickRate(u8),
    EntityId(u32),
    BlackCellID(i64, i64),
    OrderedCellsMain(Vec<String>),
    OrderedCellsDetails1(Vec<String>),
    ChangeScene(bool, String),
    ServerEntityId(u32),
    RepeatingSFX(String, Vec<String>),
    FinishedInitialization
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnreliableServerMessage {
    TransformUpdate(u32, Vec3, Quat, Vec3, u64),
    PositionUpdate(u32, Vec3, u64)
}
