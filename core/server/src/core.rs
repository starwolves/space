use std::collections::HashMap;

use bevy::prelude::{Component, Entity};

/// A resource that links entities to their appropiate connection handles for connected players.
#[derive(Default)]
pub struct HandleToEntity {
    pub map: HashMap<u64, Entity>,
    pub inv_map: HashMap<Entity, u64>,
}

/// The component for an entity controlled by a connected player.
#[derive(Component, Clone)]
pub struct ConnectedPlayer {
    pub handle: u64,
    pub authid: u16,
    pub rcon: bool,
    pub connected: bool,
}
impl Default for ConnectedPlayer {
    fn default() -> Self {
        Self {
            handle: 0,
            authid: 0,
            rcon: false,
            connected: true,
        }
    }
}

/// Resource containing the tickrate of the server loop.
pub struct TickRate {
    pub physics_rate: u8,
    pub bevy_rate: u8,
}

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
pub struct ServerId {
    pub id: Entity,
}

impl Default for ServerId {
    fn default() -> Self {
        ServerId {
            id: Entity::from_raw(0),
        }
    }
}
