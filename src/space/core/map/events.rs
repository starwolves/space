use bevy::{prelude::Entity, math::Vec2};

use crate::space::core::networking::resources::ReliableServerMessage;

pub struct InputMapChangeDisplayMode{
    pub handle : u32,
    pub entity : Entity,
    pub display_mode : String,
}

pub struct InputMapRequestDisplayModes{
    pub handle : u32,
    pub entity : Entity,
}

pub struct NetRequestDisplayModes {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetDisplayAtmospherics {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct InputMap {
    pub handle : u32,
    pub entity : Entity,
    pub input : MapInput,
}


pub enum MapInput {
    Range(f32),
    Position(Vec2)
}
