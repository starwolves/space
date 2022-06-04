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

use super::JumpsuitSummoner;

impl RigidBodySummonable for JumpsuitSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.269, 0.377, 0.098),
            collider_transform: Transform::from_translation(Vec3::new(0., -0.021, -0.011)),
            collider_friction: friction,

            ..Default::default()
        }
    }
}
