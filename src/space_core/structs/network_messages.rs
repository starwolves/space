use serde::{Serialize, Deserialize};

use crate::space_core::{resources::world_environments::WorldEnvironment};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReliableServerMessage {
    ConfigMessage(ConfigMessage)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConfigMessage {
    Awoo,
    WorldEnvironment(WorldEnvironment),
    TickRate(u8),
    HandleId(u32),
    BlackCellID(i64, i64),
    ChangeScene(bool, String),
    ServerEntityId(u16)
}
