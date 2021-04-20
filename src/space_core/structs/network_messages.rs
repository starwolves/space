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
    EntityUpdate(u32,String, EntityUpdateData),
    ConfigMessage(ServerConfigMessage),
    UIRequestInput(String, String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityUpdateData {
    UIText(String)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerConfigMessage {
    Awoo,
    WorldEnvironment(WorldEnvironment),
    TickRate(u8),
    HandleId(u32),
    BlackCellID(i64, i64),
    OrderedCellsMain(Vec<String>),
    OrderedCellsDetails1(Vec<String>),
    ChangeScene(bool, String),
    ServerEntityId(u32)
}
