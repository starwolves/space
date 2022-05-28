use bevy_ecs::{entity::Entity, prelude::Component};
use bevy_math::{Quat, Vec3};
use bevy_rapier3d::prelude::CoefficientCombineRule;
use bevy_transform::components::Transform;
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

#[derive(Component)]
pub struct RigidBodyDisabled;

#[derive(Component)]
pub struct RigidBodyLinkTransform {
    pub follow_entity: Entity,
    pub active: bool,
}
impl Default for RigidBodyLinkTransform {
    fn default() -> Self {
        Self {
            follow_entity: Entity::from_raw(0),
            active: true,
        }
    }
}

#[derive(Component)]
pub struct UpdateTransform;

#[derive(Component)]
pub struct RigidBodyData {
    pub friction: f32,
    pub friction_combine_rule: CoefficientCombineRule,
}
