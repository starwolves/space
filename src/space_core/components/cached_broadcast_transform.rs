use bevy::{math::{Quat, Vec3}, prelude::Transform};

pub struct CachedBroadcastTransform {
    pub transform : Transform,
    pub is_active : bool,
}

impl CachedBroadcastTransform {

    pub fn new() -> CachedBroadcastTransform {
        CachedBroadcastTransform {
            transform : Transform{
                translation: Vec3::ZERO,
                rotation: Quat::from_rotation_x(0.),
                scale: Vec3::ZERO,
            },
            is_active: false,
        }
    }

}
