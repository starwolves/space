use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::rigid_body::spawn::RigidbodyBundle;

use super::CHARACTER_FLOOR_FRICTION;

pub const R: f32 = 0.5;

pub fn rigidbody_bundle() -> RigidbodyBundle {
    let mut friction = Friction::coefficient(CHARACTER_FLOOR_FRICTION);
    friction.combine_rule = CoefficientCombineRule::Min;

    RigidbodyBundle {
        collider: Collider::capsule(
            Vec3::new(0.0, 0.0 + R, 0.0).into(),
            Vec3::new(0.0, 1.8 - R, 0.0).into(),
            R,
        ),
        collider_transform: Transform::from_translation(Vec3::new(0., 0.011, -0.004)),
        collider_friction: friction,
        rigidbody_dynamic: false,
        ..Default::default()
    }
}
