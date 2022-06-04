use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::resources::SpawnData,
    rigid_body::spawn::{RigidBodyBundle, RigidBodySummonable},
};

use super::AirlockSummoner;

pub const DEFAULT_AIR_LOCK_Y: f32 = 1.;

impl RigidBodySummonable for AirlockSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(1., 1., 0.2),
            collider_transform: Transform::from_translation(Vec3::new(0., DEFAULT_AIR_LOCK_Y, 0.)),
            collider_friction: friction,
            rigidbody_dynamic: false,
            collision_events: true,
        }
    }
}
