use std::collections::HashMap;

use bevy::ecs::schedule::SystemSet;
use bevy::log::warn;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Entity, Query, Res, ResMut, Resource, Transform, With};
use bevy_xpbd_3d::components::{Collider, CollisionLayers, Friction, LockedAxes, Sleeping};
use bevy_xpbd_3d::prelude::{
    AngularDamping, AngularVelocity, ExternalAngularImpulse, ExternalForce, ExternalImpulse,
    ExternalTorque, LinearDamping, LinearVelocity, RigidBody,
};
use networking::stamp::TickRateStamp;
use serde::{Deserialize, Serialize};

use crate::entity::{RigidBodies, SFRigidBody};
#[derive(Resource, Default, Clone)]
pub struct PhysicsCache {
    pub cache: HashMap<u64, Vec<Cache>>,
}
#[derive(Clone)]
pub struct Cache {
    pub entity: Entity,
    pub rb_entity: Entity,
    pub linear_velocity: LinearVelocity,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
    pub angular_velocity: AngularVelocity,
    pub external_force: ExternalForce,
    pub external_torque: ExternalTorque,
    pub external_impulse: ExternalImpulse,
    pub external_angular_impulse: ExternalAngularImpulse,
    pub rigidbody: RigidBody,
    pub transform: Transform,
    pub collider: Collider,
    pub sleeping: Sleeping,
    pub collision_layers: CollisionLayers,
    pub locked_axes: LockedAxes,
    pub collider_friction: Friction,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmallCache {
    pub entity: Entity,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    pub translation: Vec3,
    pub rotation: Quat,
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PhysicsSet {
    Correct,
    Cache,
}

pub(crate) fn cache_data(
    query: Query<
        (
            (
                Entity,
                &Transform,
                &LinearVelocity,
                &LinearDamping,
                &AngularDamping,
                &AngularVelocity,
                &ExternalTorque,
                &ExternalAngularImpulse,
                &ExternalImpulse,
                &ExternalForce,
                &RigidBody,
                &Collider,
                &Sleeping,
                &LockedAxes,
                &CollisionLayers,
            ),
            &Friction,
        ),
        With<SFRigidBody>,
    >,
    stamp: Res<TickRateStamp>,
    mut cache: ResMut<PhysicsCache>,
    rigidbodies: Res<RigidBodies>,
) {
    for (t0, collider_friction) in query.iter() {
        let (
            rb_entity,
            transform,
            linear_velocity,
            linear_damping,
            angular_damping,
            angular_velocity,
            external_torque,
            external_angular_impulse,
            external_impulse,
            external_force,
            rigidbody,
            collider,
            sleeping,
            locked_axes,
            collision_layers,
        ) = t0;

        let entity;
        match rigidbodies.get_entity_rigidbody(&rb_entity) {
            Some(e) => {
                entity = *e;
            }
            None => {
                warn!("Couldnt find rb_entity entity.");
                continue;
            }
        }

        let ncache = Cache {
            entity,
            rb_entity,
            linear_velocity: *linear_velocity,
            transform: *transform,
            external_torque: *external_torque,
            linear_damping: *linear_damping,
            angular_damping: *angular_damping,
            angular_velocity: *angular_velocity,
            external_force: *external_force,
            external_impulse: *external_impulse,
            external_angular_impulse: *external_angular_impulse,
            rigidbody: *rigidbody,
            collider: collider.clone(),
            sleeping: *sleeping,
            collision_layers: *collision_layers,
            locked_axes: *locked_axes,
            collider_friction: *collider_friction,
        };

        match cache.cache.get_mut(&stamp.large) {
            Some(c) => {
                c.push(ncache);
            }
            None => {
                cache.cache.insert(stamp.large, vec![ncache]);
            }
        }
    }

    // Clean cache.
    let mut to_remove = vec![];
    for recorded_stamp in cache.cache.keys() {
        if stamp.large >= 256 && recorded_stamp < &(stamp.large - 256) {
            to_remove.push(*recorded_stamp);
        }
    }
    for i in to_remove {
        cache.cache.remove(&i);
    }
}
