use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::resources::SpawnData,
    rigid_body::{
        components::STANDARD_BODY_FRICTION,
        spawn::{RigidBodyBundle, RigidBodySummonable},
    },
};

use super::HelmetSummoner;

impl RigidBodySummonable for HelmetSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.208, 0.277, 0.213),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.011, -0.004)),
            collider_friction: friction,

            ..Default::default()
        }
    }
}
