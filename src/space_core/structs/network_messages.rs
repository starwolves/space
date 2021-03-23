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
    TickRate(u8)
}
