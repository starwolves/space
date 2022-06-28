use bevy_ecs::entity::Entity;
use bevy_math::Vec2;

use crate::core::networking::resources::ReliableServerMessage;

pub struct InputMapChangeDisplayMode {
    pub handle: u64,
    pub entity: Entity,
    pub display_mode: String,
}

pub struct InputMapRequestDisplayModes {
    pub handle: u64,
    pub entity: Entity,
}

pub struct NetRequestDisplayModes {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct InputMap {
    pub handle: u64,
    pub entity: Entity,
    pub input: MapInput,
}

pub enum MapInput {
    Range(f32),
    Position(Vec2),
    MouseCell(i16, i16),
}
