use bevy::prelude::{Component, Vec3};

/// Component holding Godot GIProbe properties.
#[derive(Component, Clone)]
#[cfg(feature = "server")]
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
