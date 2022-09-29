use bevy::{
    math::Vec3,
    prelude::{Component, Entity},
};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Default)]
pub struct Vec2Int {
    pub x: i16,
    pub y: i16,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Vec3Int {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub const PISTOL_L1_ENTITY_NAME: &str = "pistolL1";
pub const JUMPSUIT_SECURITY_ENTITY_NAME: &str = "jumpsuitSecurity";

pub const HUMAN_DUMMY_ENTITY_NAME: &str = "humanDummy";
pub const HUMAN_MALE_ENTITY_NAME: &str = "humanMale";

/// Component holding Godot GIProbe properties.
#[derive(Component, Clone)]
pub struct GIProbe {
    pub bias: f32,
    pub compressed: bool,
    pub dynamic_range: u8,
    pub energy: f32,
    pub interior: bool,
    pub normal_bias: f32,
    pub propagation: f32,
    pub subdiv: u8,
    pub extents: Vec3,
}
/// Component holding Godot ReflectionProbe properties.
#[derive(Component, Clone)]
pub struct ReflectionProbe {
    pub projection_enabled: bool,
    pub cull_mask: i64,
    pub shadows_enabled: bool,
    pub extents: Vec3,
    pub intensity: f32,
    pub interior_ambient: (f32, f32, f32, f32),
    pub interior_ambient_probe_contribution: f32,
    pub interior_ambient_energy: f32,
    pub set_as_interior: bool,
    pub max_distance: f32,
    pub origin_offset: Vec3,
    pub update_mode: u8,
}

pub struct NoData;
pub enum LockedStatus {
    Open,
    Closed,
    None,
}
/// Air lock open request event.
pub struct AirLockCloseRequest {
    pub interacter_option: Option<Entity>,
    pub interacted: Entity,
}
