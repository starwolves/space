use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::rigid_body::{components::STANDARD_BODY_FRICTION, spawn::RigidbodyBundle};

pub fn rigidbody_bundle() -> RigidbodyBundle {
    let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
    friction.combine_rule = CoefficientCombineRule::Multiply;

    RigidbodyBundle {
        collider: Collider::cuboid(0.269, 0.377, 0.098),
        collider_transform: Transform::from_translation(Vec3::new(0., -0.021, -0.011)),
        collider_friction: friction,
    }
}
