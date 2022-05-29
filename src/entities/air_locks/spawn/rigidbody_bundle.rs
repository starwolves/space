use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::rigid_body::spawn::RigidbodyBundle;

use super::DEFAULT_AIR_LOCK_Y;

pub fn rigidbody_bundle() -> RigidbodyBundle {
    let mut friction = Friction::coefficient(0.);
    friction.combine_rule = CoefficientCombineRule::Multiply;

    RigidbodyBundle {
        collider: Collider::cuboid(1., 1., 0.2),
        collider_transform: Transform::from_translation(Vec3::new(0., DEFAULT_AIR_LOCK_Y, 0.)),
        collider_friction: friction,
        rigidbody_dynamic: false,
        collision_events: true,
    }
}
