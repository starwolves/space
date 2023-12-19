use std::collections::HashMap;

use bevy::ecs::entity::Entity;
use bevy::ecs::event::Event;
use bevy::ecs::query::{With, Without};
use bevy::ecs::schedule::SystemSet;
use bevy::ecs::system::{Commands, Query};
use bevy::log::{info, warn};
use bevy::transform::components::Transform;
use bevy::{
    prelude::{EventReader, EventWriter, Local, Res, ResMut, Resource},
    time::{Fixed, Time},
};

use bevy_xpbd_3d::components::{
    AngularDamping, AngularVelocity, CollisionLayers, ExternalAngularImpulse, ExternalForce,
    ExternalImpulse, ExternalTorque, Friction, LinearDamping, LinearVelocity, LockedAxes,
    RigidBody, Sleeping,
};
use bevy_xpbd_3d::prelude::{Physics, PhysicsTime};
use entity::despawn::DespawnEntity;
use entity::entity_types::BoxedEntityType;
use entity::spawn::ServerEntityClientEntity;
use networking::client::IncomingUnreliableServerMessage;
use networking::server::{ConnectedPlayer, OutgoingUnreliableServerMessage};
use networking::{
    client::{
        IncomingReliableServerMessage, NetworkingClientMessage, OutgoingReliableClientMessage,
    },
    server::{AdjustSync, NetworkingServerMessage},
    stamp::{PauseTickStep, TickRateStamp},
};
use resources::core::TickRate;
use resources::correction::{StartCorrection, SyncWorld};
use resources::grid::TileCollider;
use resources::modes::Mode;
use resources::player::SoftPlayer;

use crate::cache::{
    PhysicsCache, PriorityPhysicsCache, PriorityUpdate, SmallCache, SyncEntitiesPhysics,
};
use crate::entity::{RigidBodies, SFRigidBody};
use crate::net::PhysicsUnreliableServerMessage;
use crate::spawn::{rigidbody_builder, RigidBodyBuildData};
#[derive(Resource, Default)]
pub(crate) struct FastForwarding {
    pub forwarding: bool,
    pub advance: u16,
    pub i: u16,
}

pub const DEBUG_FAST_FORWARD: bool = false;

#[derive(Resource, Default)]
pub struct SyncPause {
    pub duration: u16,
    pub i: u16,
}

#[derive(Resource, Default)]
pub struct ClientStartedSyncing(pub bool);

pub(crate) fn start_sync(
    mut out: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut stamp: ResMut<TickRateStamp>,
    mut tickrate: ResMut<TickRate>,
    mut physics_loop: ResMut<Time<Physics>>,
    mut start: ResMut<ClientStartedSyncing>,
) {
    for message in out.read() {
        match &message.message {
            NetworkingServerMessage::StartSync(start_sync) => {
                *stamp = start_sync.stamp.clone();
                *tickrate = start_sync.tick_rate.clone();
                physics_loop.unpause();
                start.0 = true;
            }
            _ => (),
        }
    }
}

pub(crate) fn pause_loop(mut physics_loop: ResMut<Time<Physics>>) {
    physics_loop.pause();
}

pub(crate) fn sync_loop(
    mut net: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut physics_loop: ResMut<Time<Physics>>,
    mut paused: ResMut<SyncPause>,
    mut sync_queue: Local<Vec<AdjustSync>>,
    mut out: EventWriter<OutgoingReliableClientMessage<NetworkingClientMessage>>,
    mut fixed_time: ResMut<Time<Fixed>>,
    mut fast_forwarding: ResMut<FastForwarding>,
    mut p: ResMut<PauseTickStep>,
    stamp: Res<TickRateStamp>,
) {
    if physics_loop.is_paused() {
        paused.i += 1;
        if paused.i >= paused.duration {
            physics_loop.unpause();
            out.send(OutgoingReliableClientMessage {
                message: NetworkingClientMessage::SyncConfirmation,
            });
        }
    } else if fast_forwarding.forwarding {
        fast_forwarding.i += 1;
        if fast_forwarding.i >= fast_forwarding.advance {
            fast_forwarding.forwarding = false;
            fixed_time.set_timestep_seconds(1. / TickRate::default().fixed_rate as f64);
            out.send(OutgoingReliableClientMessage {
                message: NetworkingClientMessage::SyncConfirmation,
            });
            p.0 = false;
        }
    }

    let mut adjustment_option = None;

    let process_queue;

    match sync_queue.get(0) {
        Some(adjustment) => {
            process_queue = true;
            adjustment_option = Some(adjustment.clone());
        }
        None => {
            process_queue = false;
        }
    }

    for message in net.read() {
        match &message.message {
            NetworkingServerMessage::AdjustSync(adjustment) => {
                if !process_queue && adjustment_option.is_none() {
                    adjustment_option = Some(adjustment.clone());
                } else {
                    sync_queue.push(adjustment.clone());
                }
            }
            _ => (),
        }
    }

    let mut erase_queue = false;

    match adjustment_option {
        Some(adjustment) => {
            if !physics_loop.is_paused() {
                let delta = (((stamp.iteration as i128 - adjustment.iteration as i128)
                    * u8::MAX as i128)
                    + adjustment.tick as i128) as i16;

                if delta > 0 {
                    paused.duration = delta as u16;
                    paused.i = 0;
                    physics_loop.pause();
                    if process_queue {
                        erase_queue = true;
                        info!("- {} ticks (from queue)", paused.duration);
                    } else {
                        info!("- {} ticks", paused.duration);
                    }
                } else {
                    if process_queue {
                        info!("+ {} ticks (from queue)", delta.abs());
                    } else {
                        info!("+ {} ticks", delta.abs());
                    }

                    fixed_time.set_timestep_seconds(
                        (1. / TickRate::default().fixed_rate as f64) / (delta.abs() + 1) as f64,
                    );
                    fast_forwarding.forwarding = true;
                    fast_forwarding.i = 0;
                    fast_forwarding.advance = delta.abs() as u16;
                    if DEBUG_FAST_FORWARD {
                        p.0 = true;
                    }
                }
            } else if !process_queue {
                sync_queue.push(adjustment.clone());
            }
        }
        None => {}
    }
    if erase_queue {
        sync_queue.remove(0);
    }
}

/// Correction server system.
pub fn correction_sync_physics_data(
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
    sync: Res<SyncWorld>,
    cache: Res<PhysicsCache>,
    correction: Res<StartCorrection>,
    mut commands: Commands,
) {
    if sync.second_tick {
        match cache.cache.get(&correction.start_tick) {
            Some(sync_tick_cache) => {
                for (_, cache) in sync_tick_cache.iter() {
                    match query.get_mut(cache.entity) {
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
                            //warn!("Missed sync1 for {:?}", cache.entity);
                        }
                    }
                }
            }
            None => {}
        }
    }
}
#[derive(Resource, Default)]
pub struct CorrectionServerRigidBodyLink {
    // Sim entity, client entity.
    pub map: HashMap<Entity, Entity>,
}

/// Sync entities on the correction server.
pub(crate) fn sync_correction_world_entities(
    cache: Res<PhysicsCache>,
    sync: Res<SyncWorld>,
    query: Query<Entity, (With<SFRigidBody>, Without<TileCollider>)>,
    mut despawn: EventWriter<DespawnEntity>,
    mut commands: Commands,
    mut rigid_bodies: ResMut<RigidBodies>,
    app_mode: Res<Mode>,
    mut link: ResMut<CorrectionServerRigidBodyLink>,
    correction: Res<StartCorrection>,
    mut event: EventWriter<SpawningSimulationRigidBody>,
) {
    if sync.rebuild {
        match cache.cache.get(&correction.start_tick) {
            Some(sync_tick_cache) => {
                // Despawn remainder entities.
                for q in query.iter() {
                    let mut found = false;
                    for (_, c) in sync_tick_cache.iter() {
                        if c.entity == q {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        match link.map.get(&q) {
                            Some(cid) => {
                                info!("Correction despawn {:?}, cid:{:?}", q, cid);
                            }
                            None => {
                                warn!("Correction despawn (nolink) {:?}", q);
                            }
                        }
                        link.map.remove(&q);
                        despawn.send(DespawnEntity { entity: q });
                    }
                }
                // Spawn new entities.
                for (_, c) in sync_tick_cache.iter() {
                    let mut found = false;
                    for q in query.iter() {
                        if c.entity == q {
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        // Strictly spawn rigidbodies.
                        // Try to manually call rigidbodybuilder and spawn function. Dont use SpawnEntity.

                        let entity = commands.spawn(()).id();
                        link.map.insert(entity, c.entity);
                        let dynamic;
                        match c.rigidbody {
                            RigidBody::Dynamic => {
                                dynamic = true;
                            }
                            _ => {
                                dynamic = false;
                            }
                        }

                        rigidbody_builder(
                            &mut commands,
                            RigidBodyBuildData {
                                rigidbody_dynamic: dynamic,
                                rigid_transform: c.transform,
                                external_force: c.external_force,
                                linear_velocity: c.linear_velocity,
                                sleeping: c.sleeping,
                                entity_is_stored_item: false,
                                collider: c.collider.clone(),
                                friction: c.collider_friction,
                                collider_collision_layers: c.collision_layers,
                                collision_events: false,
                                mesh_offset: Transform::default(),
                                locked_axes: c.locked_axes,
                                linear_damping: c.linear_damping,
                                angular_damping: c.angular_damping,
                                angular_velocity: c.angular_velocity,
                                external_torque: c.external_torque,
                                external_angular_impulse: c.external_angular_impulse,
                                external_impulse: c.external_impulse,
                            },
                            entity,
                            false,
                            &mut rigid_bodies,
                            &app_mode,
                        );
                        event.send(SpawningSimulationRigidBody {
                            entity,
                            entity_type: c.entity_type.clone(),
                        });
                        info!("Correction spawn {:?}, cid:{:?}", entity, c.entity);
                    }
                }
            }
            None => {
                //warn!("Missed cache ({})", correction.start_tick,);
            }
        }

        // Correct the data of still existing physics entities now.
    }
}

/// Send low frequency rigidbody data to clients for transform and velocities desync checks.
pub(crate) fn send_desync_check(
    query: Query<(Entity, &Transform, &LinearVelocity, &AngularVelocity), With<SFRigidBody>>,
    rigid_bodies: Res<RigidBodies>,
    mut net: EventWriter<OutgoingUnreliableServerMessage<PhysicsUnreliableServerMessage>>,
    players: Query<&ConnectedPlayer, Without<SoftPlayer>>,
    mut local: Local<u8>,
    rate: Res<TickRate>,
) {
    *local += 1;
    if *local as f32 >= rate.fixed_rate as f32 / 4. {
        *local = 0;
    } else {
        return;
    }
    let mut small_cache = vec![];
    for (rb_entity, transform, linear_velocity, angular_velocity) in query.iter() {
        match rigid_bodies.get_rigidbody_entity(&rb_entity) {
            Some(entity) => {
                small_cache.push(SmallCache {
                    entity: *entity,
                    linear_velocity: linear_velocity.0,
                    angular_velocity: angular_velocity.0,
                    translation: transform.translation,
                    rotation: transform.rotation,
                });
            }
            None => {
                //warn!("Couldnt find rigidbody entity. {:?}", rb_entity);
            }
        }
    }
    if small_cache.len() > 0 {
        for c in players.iter() {
            if c.connected {
                net.send(OutgoingUnreliableServerMessage {
                    message: PhysicsUnreliableServerMessage::DesyncCheck(small_cache.clone()),
                    handle: c.handle,
                });
            }
        }
    }
}

#[derive(Event)]
pub struct SpawningSimulationRigidBody {
    pub entity: Entity,
    pub entity_type: BoxedEntityType,
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpawningSimulation {
    Spawn,
}
#[derive(Resource, Default)]
pub struct PendingDesync(Option<(u64, PhysicsUnreliableServerMessage)>);

pub(crate) fn desync_check_correction(
    mut messages: EventReader<IncomingUnreliableServerMessage<PhysicsUnreliableServerMessage>>,
    mut cache: ResMut<PhysicsCache>,
    mut correction: EventWriter<StartCorrection>,
    stamp: Res<TickRateStamp>,
    server_client_entity: Res<ServerEntityClientEntity>,
    mut latest_desync: ResMut<PendingDesync>,
    mut syncs: EventWriter<SyncEntitiesPhysics>,
    mut priority: ResMut<PriorityPhysicsCache>,
) {
    for message in messages.read() {
        let message_stamp = message.stamp;
        match &message.message {
            PhysicsUnreliableServerMessage::DesyncCheck(_) => match &latest_desync.0 {
                Some((latest_stamp, _)) => {
                    if *latest_stamp >= message_stamp {
                        continue;
                    }
                    if latest_stamp <= &stamp.large && message_stamp > stamp.large {
                        continue;
                    }
                    latest_desync.0 = Some((message_stamp, message.message.clone()));
                }
                None => {
                    latest_desync.0 = Some((message_stamp, message.message.clone()));
                }
            },
        }
    }
    let mut clear_pending = false;
    match &latest_desync.0 {
        Some((latest_stamp, message)) => {
            if latest_stamp > &stamp.large {
                return;
            }
            match cache.cache.get_mut(&latest_stamp) {
                Some(physics_cache) => match &message {
                    PhysicsUnreliableServerMessage::DesyncCheck(caches) => {
                        let mut tosync = vec![];
                        for s in caches {
                            match server_client_entity.map.get(&s.entity) {
                                Some(entity) => {
                                    for (_, c) in physics_cache.iter_mut() {
                                        if c.entity == *entity {
                                            c.angular_velocity =
                                                AngularVelocity(s.angular_velocity);
                                            c.linear_velocity = LinearVelocity(s.linear_velocity);
                                            c.transform = Transform {
                                                translation: s.translation,
                                                rotation: s.rotation,
                                                ..Default::default()
                                            };
                                            tosync.push(c.entity);
                                            match priority.cache.get_mut(&latest_stamp) {
                                                Some(cac) => {
                                                    cac.insert(
                                                        c.entity,
                                                        PriorityUpdate::SmallCache(s.clone()),
                                                    );
                                                }
                                                None => {
                                                    let mut map = HashMap::new();
                                                    map.insert(
                                                        c.entity,
                                                        PriorityUpdate::SmallCache(s.clone()),
                                                    );
                                                    priority.cache.insert(*latest_stamp, map);
                                                }
                                            }

                                            break;
                                        }
                                    }
                                }
                                None => {
                                    warn!("Couldnt find server client entity.");
                                }
                            }
                        }

                        if latest_stamp == &stamp.large {
                            info!("Perfect desync check.");
                            syncs.send(SyncEntitiesPhysics { entities: tosync });
                        } else {
                            correction.send(StartCorrection {
                                start_tick: *latest_stamp,
                                last_tick: stamp.large,
                            });
                        }
                        clear_pending = true;
                    }
                },
                None => {
                    //warn!("Missed desync check ({})", latest_desync_stamp);
                }
            }
        }
        None => {}
    }
    if clear_pending {
        latest_desync.0 = None;
    }
}

/// Correction server system.
pub fn apply_priority_cache(
    priority: Res<PriorityPhysicsCache>,
    mut query: Query<
        (&mut Transform, &mut LinearVelocity, &mut AngularVelocity),
        With<SFRigidBody>,
    >,
    stamp: Res<TickRateStamp>,
) {
    match priority.cache.get(&stamp.large) {
        Some(s) => {
            for (entity, update) in s.iter() {
                match query.get_mut(*entity) {
                    Ok((mut transform, mut linear_velocity, mut angular_velocity)) => {
                        match update {
                            PriorityUpdate::SmallCache(cache) => {
                                transform.translation = cache.translation;
                                transform.rotation = cache.rotation;
                                linear_velocity.0 = cache.linear_velocity;
                                angular_velocity.0 = cache.angular_velocity;
                            }
                            PriorityUpdate::Position(p) => {
                                transform.translation = *p;
                            }
                        }
                    }
                    Err(_) => {
                        warn!("Couldnt find priority entity.");
                    }
                }
            }
        }
        None => {}
    }
}
