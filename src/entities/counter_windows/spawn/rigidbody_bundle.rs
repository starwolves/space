use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::rigid_body::spawn::RigidbodyBundle;

use super::COUNTER_WINDOW_COLLISION_Y;

pub fn rigidbody_bundle() -> RigidbodyBundle {
    let mut friction = Friction::coefficient(0.);
    friction.combine_rule = CoefficientCombineRule::Average;

    RigidbodyBundle {
        collider: Collider::cuboid(0.1, 0.5, 1.),
        collider_transform: Transform::from_translation(Vec3::new(
            0.,
            COUNTER_WINDOW_COLLISION_Y,
            0.,
        )),
        collider_friction: friction,
        rigidbody_dynamic: false,
        ..Default::default()
    }
}
