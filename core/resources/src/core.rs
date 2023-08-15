use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

/// Resource containing the tickrate of the server loop.

#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct TickRate {
    pub physics_rate: u8,
    pub physics_substep: u8,
    pub bevy_rate: u8,
}

pub const DEFAULT_TICKRATE: TickRate = TickRate {
    physics_rate: 60,
    physics_substep: 2,
    bevy_rate: 60,
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
