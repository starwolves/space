use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::rigid_body::{components::STANDARD_BODY_FRICTION, spawn::RigidbodyBundle};

pub fn rigidbody_bundle() -> RigidbodyBundle {
    let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
    friction.combine_rule = CoefficientCombineRule::Multiply;

    RigidbodyBundle {
        collider: Collider::cuboid(0.208, 0.277, 0.213),
        collider_transform: Transform::from_translation(Vec3::new(0., 0.011, -0.004)),
        collider_friction: friction,

        ..Default::default()
    }
}
