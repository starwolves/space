use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        entity::{resources::SpawnData, spawn::NoData},
        rigid_body::{
            components::STANDARD_BODY_FRICTION,
            spawn::{RigidBodyBundle, RigidBodySummonable},
        },
    },
    entities::pistol_l1::PistolL1Summoner,
};

impl RigidBodySummonable<NoData> for PistolL1Summoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.047, 0.219, 0.199),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
            collider_friction: friction,

            ..Default::default()
        }
    }
}
