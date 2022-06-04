use bevy_ecs::prelude::Component;
use bevy_math::Vec3;

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
