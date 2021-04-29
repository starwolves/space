use std::collections::HashMap;

use bevy::{math::{Quat, Vec3}, prelude::Color};
use serde::{Serialize, Deserialize};

use crate::space_core::{resources::world_environments::WorldEnvironment};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReliableClientMessage {
    Awoo,
    UIInput(UIInputNodeClass,UIInputAction,String,String),
    SceneReady(String),
    UIInputTransmitText(String, String, String)
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
    LoadEntity(String, String, HashMap<String, HashMap<String, EntityUpdateData>>, u32, bool, String, String, bool)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityUpdateData {
    Int(i64),
    UInt8(u8),
    String(String),
    Float(f32),
    Transform(Vec3,Quat,Vec3),
    Color(Color),
    Bool(bool)
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
    ServerEntityId(u32)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnreliableServerMessage {
    TransformUpdate(u32, Vec3, Quat, Vec3, u64)
}
