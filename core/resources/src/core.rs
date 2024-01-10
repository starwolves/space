use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

/// Resource containing the tickrate of the server loop.

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct TickRate {
    pub fixed_rate: u8,
    pub physics_substep: u8,
}

pub const DEFAULT_TICKRATE: TickRate = TickRate {
    fixed_rate: 60,
    physics_substep: 1,
};

impl Default for TickRate {
    fn default() -> Self {
        DEFAULT_TICKRATE
    }
}

/// Resource that stores information of client.

#[derive(Resource)]
pub struct ClientInformation {
    pub version: String,
}
