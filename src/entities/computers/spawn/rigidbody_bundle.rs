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

use super::ComputerSummoner;

impl RigidBodySummonable<NoEntityData> for ComputerSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoEntityData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(STANDARD_BODY_FRICTION);
        friction.combine_rule = CoefficientCombineRule::Min;

        RigidBodyBundle {
            collider: Collider::cuboid(1., 0.7, 1.),
            collider_transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            collider_friction: friction,
            rigidbody_dynamic: false,
            collision_events: true,
        }
    }
}
