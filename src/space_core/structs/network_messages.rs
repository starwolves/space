use serde::{Serialize, Deserialize};

use crate::space_core::{resources::world_environments::WorldEnvironment};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReliableClientMessage {
    Awoo,
    UIInput(String,String,String,String)
}




#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReliableServerMessage {
    ConfigMessage(ServerConfigMessage)
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
    ServerEntityId(u16)
}
