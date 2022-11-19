use bevy::prelude::{Entity, Resource};

/// Resource containing the tickrate of the server loop.
#[cfg(feature = "server")]
#[derive(Resource)]
pub struct TickRate {
    pub physics_rate: u8,
    pub bevy_rate: u8,
}

#[cfg(feature = "server")]
impl Default for TickRate {
    fn default() -> Self {
        TickRate {
            physics_rate: 24,
            bevy_rate: 64,
        }
    }
}

/// Resource used for client, we can send this ID as an entityUpdate to the client which indicates it does not belong
/// to a specific entity and it should be customly assigned to something such as UIs and other stuff which
/// are not real server entities but just client GUI instances.
#[cfg(feature = "server")]
#[derive(Resource)]
pub struct ServerId {
    pub id: Entity,
}

#[cfg(feature = "server")]
impl Default for ServerId {
    fn default() -> Self {
        ServerId {
            id: Entity::from_raw(0),
        }
    }
}

/// Resource that stores information of client.
#[cfg(feature = "client")]
#[derive(Resource)]
pub struct ClientInformation {
    pub version: String,
}
