use bevy::prelude::Resource;

/// Resource containing the tickrate of the server loop.
#[cfg(feature = "server")]
#[derive(Resource)]
pub struct TickRate {
    pub physics_rate: u8,
    pub bevy_rate: u8,
}
#[cfg(feature = "server")]
pub const DEFAULT_TICKRATE: TickRate = TickRate {
    physics_rate: 64,
    bevy_rate: 64,
};

#[cfg(feature = "server")]
impl Default for TickRate {
    fn default() -> Self {
        DEFAULT_TICKRATE
    }
}

/// Resource that stores information of client.
#[cfg(feature = "client")]
#[derive(Resource)]
pub struct ClientInformation {
    pub version: String,
}
