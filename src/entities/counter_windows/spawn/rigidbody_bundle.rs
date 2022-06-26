use bevy_math::Vec3;
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{resources::SpawnData, spawn::NoEntityData},
    rigid_body::spawn::{RigidBodyBundle, RigidBodySummonable},
};

use super::{CounterWindowSummoner, COUNTER_WINDOW_COLLISION_Y};

impl RigidBodySummonable<NoEntityData> for CounterWindowSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoEntityData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Average;

        RigidBodyBundle {
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
}
