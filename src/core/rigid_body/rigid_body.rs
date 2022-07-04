use bevy::{
    math::{Quat, Vec3},
    prelude::{Component, Transform},
};
#[derive(Component)]
pub struct CachedBroadcastTransform {
    pub transform: Transform,
    pub is_active: bool,
}

impl Default for CachedBroadcastTransform {
    fn default() -> Self {
        Self {
            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Quat::from_rotation_x(0.),
                scale: Vec3::ZERO,
            },
            is_active: false,
        }
    }
}

pub const STANDARD_BODY_FRICTION: f32 = 0.125;
