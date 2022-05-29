use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::rigid_body::{components::STANDARD_BODY_FRICTION, spawn::RigidbodyBundle};

pub fn rigidbody_bundle() -> RigidbodyBundle {
    let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
    friction.combine_rule = CoefficientCombineRule::Min;

    RigidbodyBundle {
        collider: Collider::cuboid(1., 0.7, 1.),
        collider_transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        collider_friction: friction,
        rigidbody_dynamic: false,
        collision_events: true,
    }
}
