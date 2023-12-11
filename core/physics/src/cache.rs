use std::collections::HashMap;

use bevy::log::warn;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Entity, Query, Res, ResMut, Resource, Transform, With};
use bevy_xpbd_3d::components::{Collider, CollisionLayers, Friction, LockedAxes, Sleeping};
use bevy_xpbd_3d::prelude::{
    AngularDamping, AngularVelocity, ExternalAngularImpulse, ExternalForce, ExternalImpulse,
    ExternalTorque, LinearDamping, LinearVelocity, RigidBody,
};
use entity::entity_data::EntityData;
use entity::entity_types::BoxedEntityType;
use networking::stamp::TickRateStamp;
use resources::correction::MAX_CACHE_TICKS_AMNT;
use serde::{Deserialize, Serialize};

use crate::entity::{RigidBodies, SFRigidBody};
#[derive(Resource, Default, Clone)]
pub struct PhysicsCache {
    pub cache: HashMap<u64, HashMap<Entity, Cache>>,
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
    pub sleeping: Option<Sleeping>,
    pub collision_layers: CollisionLayers,
    pub locked_axes: LockedAxes,
    pub collider_friction: Friction,
    pub entity_type: BoxedEntityType,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmallCache {
    pub entity: Entity,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    pub translation: Vec3,
    pub rotation: Quat,
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
                Option<&Sleeping>,
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
    types: Query<&EntityData>,
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
        match rigidbodies.get_rigidbody_entity(&rb_entity) {
            Some(e) => {
                entity = *e;
            }
            None => {
                warn!("Couldnt find rb_entity entity.");
                continue;
            }
        }

        let entity_type;
        match types.get(entity) {
            Ok(t) => {
                entity_type = t.entity_type.clone();
            }
            Err(_) => {
                warn!("Couldnt find entity type.");
                continue;
            }
        }
        /*info!(
            "cache_data entity:{:?} {}",
            entity,
            entity_type.get_identity()
        );*/

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
            sleeping: sleeping.copied(),
            collision_layers: *collision_layers,
            locked_axes: *locked_axes,
            collider_friction: *collider_friction,
            entity_type,
        };

        match cache.cache.get_mut(&stamp.large) {
            Some(c) => {
                c.insert(ncache.entity, ncache);
            }
            None => {
                let mut m = HashMap::new();
                m.insert(ncache.entity, ncache);
                cache.cache.insert(stamp.large, m);
            }
        }
    }
    // Clean cache.
    let mut to_remove = vec![];
    for recorded_stamp in cache.cache.keys() {
        if stamp.large >= MAX_CACHE_TICKS_AMNT
            && recorded_stamp < &(stamp.large - MAX_CACHE_TICKS_AMNT)
        {
            to_remove.push(*recorded_stamp);
        }
    }
    for i in to_remove {
        cache.cache.remove(&i);
    }
}
