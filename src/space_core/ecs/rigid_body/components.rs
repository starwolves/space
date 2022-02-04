use bevy::{math::{Quat, Vec3}, prelude::{Transform, Component, Entity}};
#[derive(Component)]
pub struct CachedBroadcastTransform {
    pub transform : Transform,
    pub is_active : bool,
}

impl Default for CachedBroadcastTransform {
    fn default() -> Self {
        Self {
            transform : Transform{
                translation: Vec3::ZERO,
                rotation: Quat::from_rotation_x(0.),
                scale: Vec3::ZERO,
            },
            is_active: false,
        }
    }
}


#[derive(Component)]
pub struct DefaultTransform {

    pub transform : Transform,

}

impl Default for DefaultTransform {
    fn default() -> Self {
        Self {
            transform : Transform::identity(),
        }
    }
}

#[derive(Component)]
pub struct InterpolationPriority {
    pub priority : InterpolationPriorityStatus,
}

impl Default for InterpolationPriority {
    fn default() -> Self {
        Self {
            priority : InterpolationPriorityStatus::Low,
        }
    }
}


#[allow(dead_code)]
pub enum InterpolationPriorityStatus {
    High,
    Medium,
    Low
}

#[derive(Component)]
pub struct RigidBodyDisabled;

#[derive(Component)]
pub struct RigidBodyLinkTransform {

    pub follow_entity : Entity,
    pub active : bool,

}
impl Default for RigidBodyLinkTransform {
    fn default() -> Self {
        Self {
            follow_entity : Entity::from_raw(0),
            active : true,
        }
    }
}

#[derive(Component)]
pub struct UpdateTransform;

