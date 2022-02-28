use bevy_internal::{math::Vec2, prelude::Entity};

use crate::space::core::networking::resources::ReliableServerMessage;

pub struct InputMapChangeDisplayMode {
    pub handle: u32,
    pub entity: Entity,
    pub display_mode: String,
}

pub struct InputMapRequestDisplayModes {
    pub handle: u32,
    pub entity: Entity,
}

pub struct NetRequestDisplayModes {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct InputMap {
    pub handle: u32,
    pub entity: Entity,
    pub input: MapInput,
}

pub enum MapInput {
    Range(f32),
    Position(Vec2),
    MouseCell(i16, i16),
}
