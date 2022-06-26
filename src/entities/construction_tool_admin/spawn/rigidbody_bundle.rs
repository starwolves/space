use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{resources::SpawnData, spawn::NoEntityData},
    rigid_body::{
        components::STANDARD_BODY_FRICTION,
        spawn::{RigidBodyBundle, RigidBodySummonable},
    },
};

use super::ConstructionToolSummoner;

impl RigidBodySummonable<NoEntityData> for ConstructionToolSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoEntityData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Multiply;

        RigidBodyBundle {
            collider: Collider::cuboid(0.11 * 1.5, 0.1 * 1.5, 0.13 * 1.5),
            collider_transform: Transform::from_translation(Vec3::new(0., 0.087, 0.)),
            collider_friction: friction,
            ..Default::default()
        }
    }
}
