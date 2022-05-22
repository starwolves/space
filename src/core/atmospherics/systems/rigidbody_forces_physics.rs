// Add to shared resources with _physics,

use bevy_ecs::{
    prelude::Without,
    system::{Query, ResMut},
};
use bevy_math::Vec3;
use bevy_rapier3d::prelude::ExternalForce;

use crate::core::{
    atmospherics::resources::RigidBodyForcesAccumulation, rigid_body::components::RigidBodyDisabled,
};

pub fn rigidbody_forces_physics(
    mut forces_accumulation: ResMut<RigidBodyForcesAccumulation>,
    mut rigidbodies: Query<&mut ExternalForce, Without<RigidBodyDisabled>>,
) {
    for (entity, accumulated) in &mut forces_accumulation.data {
        let mut net_force = Vec3::ZERO;

        for accumulation in accumulated.iter() {
            net_force += *accumulation;
        }

        match rigidbodies.get_mut(*entity) {
            Ok(mut rigid_body_forces_component) => {
                rigid_body_forces_component.force = net_force.into();
            }
            Err(_rr) => {}
        }

        accumulated.clear();
    }
}
