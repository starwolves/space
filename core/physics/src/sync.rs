use std::collections::HashMap;

use bevy::ecs::component::Component;
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
use entity::net::EntityServerMessage;
use entity::spawn::ServerEntityClientEntity;
use networking::client::{IncomingUnreliableServerMessage, TickLatency, TotalAdjustment};
use networking::server::{ConnectedPlayer, HandleToEntity, OutgoingUnreliableServerMessage};
use networking::{
    client::{
        IncomingReliableServerMessage, NetworkingClientMessage, OutgoingReliableClientMessage,
    },
    server::{AdjustSync, NetworkingServerMessage},
    stamp::{PauseTickStep, TickRateStamp},
};
use resources::core::TickRate;
use resources::correction::{IsCorrecting, StartCorrection};
use resources::grid::{GridmapCollider, Tile};
use resources::modes::AppMode;
use resources::physics::{PriorityPhysicsCache, PriorityUpdate, SmallCache};
use resources::player::SoftPlayer;

use crate::cache::{PhysicsCache, SyncEntitiesPhysics};
use crate::entity::{RigidBodies, RigidBodyLink, SFRigidBody};
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
    pub paused: bool,
}

#[derive(Resource, Default)]
pub struct ClientStartedSyncing(pub bool);

pub(crate) fn start_sync(
    mut out: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut net: EventWriter<OutgoingReliableClientMessage<NetworkingClientMessage>>,
    mut stamp: ResMut<TickRateStamp>,
    mut tickrate: ResMut<TickRate>,
    mut start: ResMut<ClientStartedSyncing>,
    mut i: Local<u16>,
) {
    for message in out.read() {
        match &message.message {
            NetworkingServerMessage::StartSync(start_sync) => {
                *stamp = start_sync.stamp.clone();
                *tickrate = start_sync.tick_rate.clone();
                start.0 = true;
                *i = 0;
                net.send(OutgoingReliableClientMessage {
                    message: NetworkingClientMessage::StartSyncConfirmation,
                });
            }
            _ => (),
        }
    }
}

pub(crate) fn sync_loop(
    mut net: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut physics_loop: ResMut<Time<Physics>>,
    mut paused: ResMut<SyncPause>,
    mut sync_queue: Local<Vec<(AdjustSync, u32)>>,
    mut out: EventWriter<OutgoingReliableClientMessage<NetworkingClientMessage>>,
    mut fixed_time: ResMut<Time<Fixed>>,
    mut fast_forwarding: ResMut<FastForwarding>,
    mut p: ResMut<PauseTickStep>,
    mut latency: ResMut<TotalAdjustment>,
    stamp: Res<TickRateStamp>,
    tick_latency: Res<TickLatency>,
) {
    if paused.paused {
        paused.i += 1;
        if paused.i >= paused.duration {
            physics_loop.unpause();
            out.send(OutgoingReliableClientMessage {
                message: NetworkingClientMessage::SyncConfirmation,
            });
            paused.paused = false;
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
                    adjustment_option = Some((adjustment.clone(), message.stamp));
                } else {
                    sync_queue.push((adjustment.clone(), message.stamp));
                }
            }
            _ => (),
        }
    }

    let mut erase_queue = false;

    match adjustment_option {
        Some((adjustment, server_stamp)) => {
            if !paused.paused {
                let new_desired_large_tick = tick_latency.latency as u32 + adjustment.tick;
                let mut delta = (stamp.tick as i64 - new_desired_large_tick as i64) as i16;
                if delta == 0 {
                    if adjustment.forward {
                        delta = -1;
                    } else {
                        delta = 1;
                    }
                }
                if delta > 0 {
                    paused.duration = delta as u16;
                    paused.i = 0;
                    physics_loop.pause();
                    paused.paused = true;
                    if process_queue {
                        erase_queue = true;
                        info!("- {} ticks (from queue)", paused.duration);
                    } else {
                        info!("- {} ticks", paused.duration);
                    }
                    latency.latency -= paused.duration as i16;
                } else {
                    if process_queue {
                        erase_queue = true;
                        info!("+ {} ticks (from queue)", delta.abs());
                    } else {
                        info!("+ {} ticks", delta.abs());
                    }
                    latency.latency += delta.abs() as i16;

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
                sync_queue.push((adjustment.clone(), server_stamp));
            }
        }
        None => {}
    }
    if erase_queue {
        sync_queue.remove(0);
    }
}

/// Correction server system.
pub fn init_physics_data(
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
    cache: Res<PhysicsCache>,
    mut commands: Commands,
    link: Res<CorrectionServerRigidBodyLink>,
    stamp: Res<TickRateStamp>,
    start: Res<StartCorrection>,
) {
    if stamp.tick == start.start_tick {
        match cache.cache.get(&stamp.tick) {
            Some(physics_cache) => {
                for (_, cache) in physics_cache.iter() {
                    if cache.spawn_frame {
                        continue;
                    }
                    let client_entity;
                    match link.get_client(&cache.entity) {
                        Some(c) => {
                            client_entity = *c;
                        }
                        None => {
                            client_entity = cache.entity;
                        }
                    }
                    match link.map.get(&client_entity) {
                        Some(sims) => match sims.active_link {
                            Some(sim) => match query.get_mut(sim) {
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
                                        commands.entity(sim).insert(Sleeping);
                                    } else {
                                        commands.entity(sim).remove::<Sleeping>();
                                    }
                                }
                                Err(_) => {}
                            },
                            None => {
                                warn!("no active link found.");
                            }
                        },
                        None => {
                            warn!("Couldnt find link pd");
                        }
                    }
                }
            }
            None => {
                //warn!("no data.");
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct LinkData {
    pub known_links: Vec<Entity>,
    pub active_link: Option<Entity>,
}

#[derive(Resource, Default, Clone)]
pub struct CorrectionServerRigidBodyLink {
    // Client entity, sim entities (they get (de)spawned acrosss time, varying IDs.)
    pub map: HashMap<Entity, LinkData>,
    pub sim_client_link: HashMap<Entity, Entity>,
    pub despawned_sims: Vec<Entity>,
}
impl CorrectionServerRigidBodyLink {
    pub fn get_client(&self, entity: &Entity) -> Option<&Entity> {
        self.sim_client_link.get(entity)
    }
    pub fn sim_despawned(&mut self, sim: Entity) {
        self.despawned_sims.push(sim);
    }
    pub fn clean_despawned(&mut self) {
        for sim in self.despawned_sims.iter() {
            match self.sim_client_link.get(sim) {
                Some(e) => match self.map.get_mut(e) {
                    Some(data) => match data.known_links.iter().position(|x| x == sim) {
                        Some(i) => {
                            data.known_links.remove(i);
                        }
                        None => {
                            warn!("to be despanwed link not found");
                        }
                    },
                    None => {
                        warn!("sim_client_link link not found");
                    }
                },
                None => {
                    warn!("despawned sim not found in sim_client_link");
                }
            }
        }
        self.despawned_sims.clear();
    }
    pub fn new_sim(&mut self, sim: Entity, client_entity: Entity) {
        match self.map.get_mut(&client_entity) {
            Some(l) => {
                l.known_links.push(sim);
                l.active_link = Some(sim);
            }
            None => {
                self.map.insert(
                    client_entity,
                    LinkData {
                        known_links: vec![sim],
                        active_link: Some(sim),
                    },
                );
            }
        }
        self.sim_client_link.insert(sim, client_entity);
    }
    pub fn get_sims(&self, entity: &Entity) -> Option<&Vec<Entity>> {
        match self.get_client(entity) {
            Some(e) => Some(&self.map.get(e).unwrap().known_links),
            None => None,
        }
    }
}

#[derive(Resource, Default)]
pub struct SimulationStorage(pub PhysicsCache);
/// Sync entities on the correction server.
pub(crate) fn sync_correction_world_entities(
    cache: Res<PhysicsCache>,
    query: Query<Entity, (With<SFRigidBody>, Without<Tile>)>,
    mut despawn: EventWriter<DespawnEntity>,
    mut commands: Commands,
    mut rigid_bodies: ResMut<RigidBodies>,
    app_mode: Res<AppMode>,
    mut link: ResMut<CorrectionServerRigidBodyLink>,
    mut event: EventWriter<SpawningSimulationRigidBody>,
    stamp: Res<TickRateStamp>,
    correcting: Res<IsCorrecting>,
) {
    let cache_tick = stamp.tick;
    if !correcting.0 {
        return;
    }

    let mut spawns = vec![];

    match cache.cache.get(&cache_tick) {
        Some(physics_cache) => {
            // Despawn remainder entities.
            for correction_entity in query.iter() {
                let mut found = false;
                let sims;
                let client_entity;
                match link.get_client(&correction_entity) {
                    Some(client) => {
                        client_entity = *client;
                        sims = link.map.get(&client).unwrap();
                    }
                    None => {
                        warn!("not found.");
                        continue;
                    }
                }
                let mut spawn_frame = false;
                match sims.active_link {
                    Some(s) => match physics_cache.get(&s) {
                        Some(c) => {
                            found = true;
                            spawn_frame = c.spawn_frame;
                        }
                        None => {}
                    },
                    None => {}
                }
                if !found {
                    match physics_cache.get(&client_entity) {
                        Some(c) => {
                            found = true;
                            spawn_frame = c.spawn_frame;
                        }
                        None => {}
                    }
                }
                if !found || spawn_frame {
                    /*match link.get_client(&correction_entity) {
                        Some(cid) => {
                            info!(
                                "Tick {} correction despawn {:?}, cid:{:?}",
                                stamp.tick, correction_entity, cid
                            );
                        }
                        None => {
                            warn!(
                                "Tick {} correction despawn (nolink) {:?}",
                                stamp.tick, correction_entity
                            );
                        }
                    }*/
                    link.map.get_mut(&client_entity).unwrap().active_link = None;
                    link.sim_despawned(correction_entity);
                    despawn.send(DespawnEntity {
                        entity: correction_entity,
                    });
                }
            }
            // Spawn loaded entities.
            for (_, ncache) in physics_cache.iter() {
                let sims;
                let client_entity;
                let mut found = false;

                match link.get_client(&ncache.entity) {
                    Some(client) => {
                        sims = link.map.get(client).unwrap().clone();
                        client_entity = *client;
                    }
                    None => {
                        client_entity = ncache.entity;
                        match link.map.get(&client_entity) {
                            Some(s) => {
                                sims = s.clone();
                            }
                            None => {
                                sims = LinkData::default();
                            }
                        }
                    }
                }

                match sims.active_link {
                    Some(e) => {
                        found = query.get(e).is_ok();
                    }
                    None => {}
                }
                if !found || ncache.spawn_frame {
                    spawns.push((client_entity, ncache.clone()));
                }
            }
        }
        None => {
            //warn!("Missed cache ({})", cache_tick,);
        }
    }

    for (client_entity, ncache) in spawns {
        // Strictly spawn rigidbodies.
        // Try to manually call rigidbodybuilder and spawn function. Dont use SpawnEntity.

        let entity = commands.spawn(()).id();
        link.new_sim(entity, client_entity);

        let dynamic;
        match ncache.rigidbody {
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
                rigid_transform: ncache.transform,
                external_force: ncache.external_force,
                linear_velocity: ncache.linear_velocity,
                sleeping: ncache.sleeping,
                entity_is_stored_item: false,
                collider: ncache.collider.clone(),
                friction: ncache.collider_friction,
                collider_collision_layers: ncache.collision_layers,
                collision_events: false,
                mesh_offset: Transform::default(),
                locked_axes: ncache.locked_axes,
                linear_damping: ncache.linear_damping,
                angular_damping: ncache.angular_damping,
                angular_velocity: ncache.angular_velocity,
                external_torque: ncache.external_torque,
                external_angular_impulse: ncache.external_angular_impulse,
                external_impulse: ncache.external_impulse,
            },
            entity,
            false,
            &mut rigid_bodies,
            &app_mode,
        );
        event.send(SpawningSimulationRigidBody {
            entity,
            entity_type: ncache.entity_type.clone(),
        });
        /*info!(
            "Tick {} correction spawn {:?}, cid:{:?}, ncache.entity: {:?} ",
            stamp.tick, entity, client_entity, ncache.entity,
        );*/
    }
}
pub const DESYNC_FREQUENCY: f32 = 4.;
pub const HIGH_ENTITIES_AMOUNT: usize = 256;
pub const MID_ENTITIES_AMOUNT: usize = 128;
#[derive(Component)]
pub struct DisableSync;
/// Send low frequency rigidbody data to clients for transform and velocities desync checks.
pub(crate) fn send_desync_check(
    query: Query<(Entity, &Transform, &LinearVelocity, &AngularVelocity), With<SFRigidBody>>,
    pawn_query: Query<Option<&DisableSync>, (With<RigidBodyLink>, Without<GridmapCollider>)>,
    rigid_bodies: Res<RigidBodies>,
    mut net: EventWriter<OutgoingUnreliableServerMessage<PhysicsUnreliableServerMessage>>,
    players: Query<&ConnectedPlayer, Without<SoftPlayer>>,
    mut local: Local<u8>,
    rate: Res<TickRate>,
    handle_to_entity: Res<HandleToEntity>,
    mut second: Local<bool>,
    mut single: Local<(bool, u8)>,
) {
    *local += 1;
    if *local as f32 >= rate.fixed_rate as f32 / DESYNC_FREQUENCY {
        single.0 = false;
        *local = 0;
        *second = !*second;
        if single.1 > DESYNC_FREQUENCY as u8 {
            single.1 = 0;
            single.0 = true;
        } else {
            single.1 += 1;
        }
    } else {
        return;
    }
    for connected_player in players.iter() {
        if !connected_player.connected {
            continue;
        }
        let player_entity;
        match handle_to_entity.map.get(&connected_player.handle) {
            Some(ent) => {
                player_entity = *ent;
            }
            None => {
                warn!("no handle entity found.3");
                continue;
            }
        }

        let mut small_cache = vec![];
        for (rb_entity, transform, linear_velocity, angular_velocity) in query.iter() {
            match rigid_bodies.get_rigidbody_entity(&rb_entity) {
                Some(entity) => {
                    let disabled;
                    match pawn_query.get(*entity) {
                        Ok(d) => {
                            disabled = d;
                        }
                        Err(_) => {
                            warn!("Couldnt find pawn query.");
                            continue;
                        }
                    }
                    if disabled.is_some() && *entity != player_entity {
                        continue;
                    }

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
        let l = small_cache.len();
        if l > HIGH_ENTITIES_AMOUNT {
            if !single.0 {
                continue;
            }
        } else if l > MID_ENTITIES_AMOUNT {
            if !*second {
                continue;
            }
        }
        if l > 0 {
            if connected_player.connected {
                net.send(OutgoingUnreliableServerMessage {
                    message: PhysicsUnreliableServerMessage::DesyncCheck(small_cache.clone()),
                    handle: connected_player.handle,
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
pub(crate) fn desync_check_correction(
    mut messages: EventReader<IncomingUnreliableServerMessage<PhysicsUnreliableServerMessage>>,
    mut cache: ResMut<PhysicsCache>,
    mut correction: EventWriter<StartCorrection>,
    stamp: Res<TickRateStamp>,
    server_client_entity: Res<ServerEntityClientEntity>,
    mut syncs: EventWriter<SyncEntitiesPhysics>,
    mut priority: ResMut<PriorityPhysicsCache>,
) {
    for message in messages.read() {
        let mut adjusted_latest = message.stamp;
        if adjusted_latest > 0 {
            adjusted_latest -= 1;
        }
        match cache.cache.get_mut(&adjusted_latest) {
            Some(physics_cache) => match &message.message {
                PhysicsUnreliableServerMessage::DesyncCheck(caches) => {
                    let mut tosync = vec![];
                    for s in caches {
                        match server_client_entity.map.get(&s.entity) {
                            Some(entity) => {
                                for (_, c) in physics_cache.iter_mut() {
                                    if c.entity == *entity {
                                        if c.spawn_frame {
                                            break;
                                        }
                                        c.angular_velocity = AngularVelocity(s.angular_velocity);
                                        c.linear_velocity = LinearVelocity(s.linear_velocity);
                                        c.transform = Transform {
                                            translation: s.translation,
                                            rotation: s.rotation,
                                            ..Default::default()
                                        };
                                        tosync.push(c.entity);
                                        match priority.cache.get_mut(&adjusted_latest) {
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
                                                priority.cache.insert(adjusted_latest, map);
                                            }
                                        }

                                        break;
                                    }
                                }
                            }
                            None => {
                                warn!("Couldnt find server client entity.");
                                continue;
                            }
                        }
                    }

                    if message.stamp == stamp.tick {
                        info!("Perfect desync check.");
                        syncs.send(SyncEntitiesPhysics { entities: tosync });
                    } else {
                        correction.send(StartCorrection {
                            start_tick: adjusted_latest,
                            last_tick: stamp.tick,
                        });
                    }
                }
            },
            None => {
                //warn!("Missed desync check ({})", adjusted_latest);
            }
        }
    }
}

pub(crate) fn client_apply_priority_cache(
    priority: Res<PriorityPhysicsCache>,
    mut query: Query<
        (&mut Transform, &mut LinearVelocity, &mut AngularVelocity),
        With<SFRigidBody>,
    >,
    stamp: Res<TickRateStamp>,
) {
    let adjusted_stamp = stamp.tick;

    match priority.cache.get(&adjusted_stamp) {
        Some(priority_cache) => {
            for (entity, update) in priority_cache.iter() {
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
                            PriorityUpdate::PhysicsSpawn(data) => {
                                transform.translation = data.translation;
                                transform.rotation = data.rotation;
                            }
                        }
                    }
                    Err(_) => {
                        warn!("Couldnt find entity in query.");
                    }
                }
            }
        }
        None => {}
    }
}

pub fn correction_server_apply_priority_cache(
    priority: Res<PriorityPhysicsCache>,
    mut query: Query<
        (&mut Transform, &mut LinearVelocity, &mut AngularVelocity),
        With<SFRigidBody>,
    >,
    stamp: Res<TickRateStamp>,
    link: Res<CorrectionServerRigidBodyLink>,
    start: Res<StartCorrection>,
) {
    if stamp.tick < start.start_tick {
        return;
    }
    let mut adjusted_stamp = stamp.tick;
    if adjusted_stamp > 0 {
        adjusted_stamp -= 1;
    }
    match priority.cache.get(&adjusted_stamp) {
        Some(priority_cache) => {
            for (entity, update) in priority_cache.iter() {
                match link.get_sims(entity) {
                    Some(sims) => {
                        let mut found = false;
                        for entity in sims.iter() {
                            match query.get_mut(*entity) {
                                Ok((mut transform, mut linear_velocity, mut angular_velocity)) => {
                                    /*info!(
                                        "Applying {:?} priority update: {:?} for tick {}",
                                        entity, update, adjusted_stamp
                                    );*/
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
                                        PriorityUpdate::PhysicsSpawn(data) => {
                                            transform.translation = data.translation;
                                            transform.rotation = data.rotation;
                                        }
                                    }
                                    found = true;
                                    break;
                                }
                                Err(_) => {}
                            }
                        }
                        if !found {
                            warn!("Couldnt find priority entity.");
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }
}

#[derive(Default, Resource)]
pub struct CorrectionEnabled(pub bool);

pub(crate) fn client_despawn_and_clean_cache(
    mut net: EventReader<IncomingReliableServerMessage<EntityServerMessage>>,
    links: Res<ServerEntityClientEntity>,
    mut cache: ResMut<PhysicsCache>,
    stamp: Res<TickRateStamp>,
    mut start: EventWriter<StartCorrection>,
) {
    for message in net.read() {
        match message.message {
            EntityServerMessage::UnloadEntity(entity) => match links.map.get(&entity) {
                Some(client_entity) => {
                    for i in message.stamp..stamp.tick {
                        match cache.cache.get_mut(&i) {
                            Some(c) => {
                                c.remove(client_entity);
                            }
                            None => {}
                        }
                    }
                    start.send(StartCorrection {
                        start_tick: message.stamp,
                        last_tick: stamp.tick,
                    });
                }
                None => {
                    warn!("Couldnt find client entity for server entity: {:?}", entity);
                }
            },
            _ => (),
        }
    }
}
