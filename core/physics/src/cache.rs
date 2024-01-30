use std::collections::HashMap;

use bevy::ecs::event::{Event, EventReader};
use bevy::ecs::system::Commands;
use bevy::log::warn;
use bevy::prelude::{Entity, Query, Res, ResMut, Resource, Transform, With};
use bevy_xpbd_3d::components::{Collider, CollisionLayers, Friction, LockedAxes, Sleeping};
use bevy_xpbd_3d::prelude::{
    AngularDamping, AngularVelocity, ExternalAngularImpulse, ExternalForce, ExternalImpulse,
    ExternalTorque, LinearDamping, LinearVelocity, RigidBody,
};
use entity::entity_data::EntityData;
use entity::entity_types::BoxedEntityType;
use entity::loading::NewToBeCachedSpawnedEntities;
use itertools::Itertools;
use networking::stamp::TickRateStamp;
use resources::correction::MAX_CACHE_TICKS_AMNT;
use resources::physics::PriorityPhysicsCache;

use crate::entity::{RigidBodies, SFRigidBody};

/// Correction server sometimes stores client-side entity ID when it is not yet spawned in.
#[derive(Resource, Default, Clone)]
pub struct PhysicsCache {
    pub cache: HashMap<u64, HashMap<Entity, Cache>>,
}
pub(crate) fn clear_physics_cache(mut cache0: ResMut<PhysicsCache>) {
    // Clean cache.
    if cache0.cache.len() > MAX_CACHE_TICKS_AMNT as usize {
        let mut j = 0;

        for i in cache0.cache.clone().keys().sorted().rev() {
            if j >= MAX_CACHE_TICKS_AMNT {
                cache0.cache.remove(i);
            }
            j += 1;
        }
    }
}
pub(crate) fn clear_priority_cache(mut cache0: ResMut<PriorityPhysicsCache>) {
    // Clean cache.
    if cache0.cache.len() > MAX_CACHE_TICKS_AMNT as usize {
        let mut j = 0;

        for i in cache0.cache.clone().keys().sorted().rev() {
            if j >= MAX_CACHE_TICKS_AMNT {
                cache0.cache.remove(i);
            }
            j += 1;
        }
    }
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
    pub spawn_frame: bool,
}

/// Apply the data of newly spawn in entities just to cache their physics properties so the correction server registers them for the first time.
pub(crate) fn apply_newly_spawned_data(
    mut new: ResMut<NewToBeCachedSpawnedEntities>,
    mut cache: ResMut<PhysicsCache>,
    stampres: Res<TickRateStamp>,
) {
    for (spawn_stamp, entity) in new.list.iter() {
        let new_data;
        match cache.cache.clone().get(&stampres.large) {
            Some(physics_cache) => match physics_cache.get(entity) {
                Some(stepped_data) => {
                    new_data = stepped_data.clone();
                }
                None => {
                    warn!(
                        "Couldnt find new spawned entity old entity cache. {:?} adjusted_tick {}",
                        entity, stampres.large
                    );
                    continue;
                }
            },
            None => {
                warn!("Couldnt find new spawned entity old cache.");
                continue;
            }
        }

        for i in *spawn_stamp..stampres.large {
            let mut this_data = new_data.clone();
            if i == *spawn_stamp {
                this_data.spawn_frame = true;
            }
            match cache.cache.get_mut(&i) {
                Some(c) => {
                    c.insert(*entity, this_data);
                }
                None => {
                    warn!("Missed cache.");
                }
            }
        }
    }
    new.list.clear();
}

/// Cache physics tick data of previous tick.
pub(crate) fn cache_data_prev_tick(
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
        let mut adjusted_stamp = stamp.large;
        if adjusted_stamp > 0 {
            adjusted_stamp -= 1;
        }

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
            spawn_frame: false,
        };

        match cache.cache.get_mut(&adjusted_stamp) {
            Some(c) => {
                match c.get(&ncache.entity) {
                    Some(x) => {
                        if x.spawn_frame {
                            continue;
                        }
                    }
                    None => {}
                }
                c.insert(ncache.entity, ncache);
            }
            None => {
                let mut m = HashMap::new();
                m.insert(ncache.entity, ncache);
                cache.cache.insert(adjusted_stamp, m);
            }
        }
    }
}

/// Cache physics tick data of this tick after PreUpdate to forward new spawned entity data physics properties to correction app.
pub(crate) fn cache_data_new_spawns(
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
        let adjusted_stamp = stamp.large;

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
            spawn_frame: false,
        };

        match cache.cache.get_mut(&adjusted_stamp) {
            Some(c) => {
                match c.get(&ncache.entity) {
                    Some(x) => {
                        if x.spawn_frame {
                            continue;
                        }
                    }
                    None => {}
                }
                c.insert(ncache.entity, ncache);
            }
            None => {
                let mut m = HashMap::new();
                m.insert(ncache.entity, ncache);
                cache.cache.insert(adjusted_stamp, m);
            }
        }
    }
}
/// Should run in preupdate and cache previous tick. This way we cache data of entities spawned with data point of last frame.
pub(crate) fn _cache_data(
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
        let adjusted_stamp = stamp.large - 1;

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
            spawn_frame: false,
        };

        match cache.cache.get_mut(&adjusted_stamp) {
            Some(c) => {
                match c.get(&ncache.entity) {
                    Some(x) => {
                        if x.spawn_frame {
                            continue;
                        }
                    }
                    None => {}
                }
                c.insert(ncache.entity, ncache);
            }
            None => {
                let mut m = HashMap::new();
                m.insert(ncache.entity, ncache);
                cache.cache.insert(adjusted_stamp, m);
            }
        }
    }
}
#[derive(Event, Default)]
pub struct SyncEntitiesPhysics {
    pub entities: Vec<Entity>,
}

pub fn sync_entities(
    mut query: Query<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut LinearDamping,
            &mut AngularDamping,
            &mut AngularVelocity,
            &mut ExternalTorque,
            &mut ExternalAngularImpulse,
            &mut ExternalImpulse,
            &mut ExternalForce,
            Option<&Sleeping>,
            &mut LockedAxes,
            &mut CollisionLayers,
            &mut Friction,
        ),
        With<SFRigidBody>,
    >,
    physics_cache: Res<PhysicsCache>,
    stamp: Res<TickRateStamp>,
    mut commands: Commands,
    mut syncs: EventReader<SyncEntitiesPhysics>,
) {
    for sync in syncs.read() {
        for entity in sync.entities.iter() {
            match physics_cache.cache.get(&(&stamp.large - 1)) {
                Some(physics_cache) => {
                    let cache;
                    match physics_cache.get(entity) {
                        Some(c) => {
                            cache = c;
                        }
                        None => {
                            warn!("Couldnt find sync cache.");
                            continue;
                        }
                    }

                    match query.get_mut(*entity) {
                        Ok((
                            mut transform,
                            mut linear_velocity,
                            mut linear_damping,
                            mut angular_damping,
                            mut angular_velocity,
                            mut external_torque,
                            mut external_angular_impulse,
                            mut external_impulse,
                            mut external_force,
                            sleeping,
                            mut locked_axes,
                            mut collision_layers,
                            mut friction,
                        )) => {
                            *transform = cache.transform;
                            *linear_velocity = cache.linear_velocity;
                            *linear_damping = cache.linear_damping;
                            *angular_damping = cache.angular_damping;
                            *angular_velocity = cache.angular_velocity;
                            *external_torque = cache.external_torque;
                            *external_angular_impulse = cache.external_angular_impulse;
                            *external_impulse = cache.external_impulse;
                            *external_force = cache.external_force;
                            *locked_axes = cache.locked_axes;
                            *collision_layers = cache.collision_layers;
                            *friction = cache.collider_friction;
                            if sleeping.is_some() {
                                commands.entity(cache.entity).insert(Sleeping);
                            } else {
                                commands.entity(cache.entity).remove::<Sleeping>();
                            }
                        }
                        Err(_) => {
                            //warn!("Missed sync for {:?}", cache.entity);
                        }
                    }
                }
                None => {}
            }
        }
    }
}
