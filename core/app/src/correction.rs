use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, SyncSender},
    thread::JoinHandle,
};

use bevy::{
    ecs::{entity::Entity, system::Commands},
    log::{error, warn},
};
use bevy::{
    ecs::{query::With, system::Query},
    log::info,
    transform::components::Transform,
};
use bevy::{
    prelude::{
        App, EventReader, EventWriter, FixedUpdate, IntoSystemConfigs, Local, NonSend, Plugin, Res,
        ResMut, Resource, Startup, Update, World,
    },
    time::{Fixed, Time},
};
use bevy_xpbd_3d::components::{
    AngularDamping, AngularVelocity, Collider, CollisionLayers, ExternalAngularImpulse,
    ExternalForce, ExternalImpulse, ExternalTorque, Friction, LinearDamping, LinearVelocity,
    LockedAxes, RigidBody, Sleeping,
};
use cameras::{LookTransform, LookTransformCache};
use controller::controller::{ControllerCache, ControllerInput};
use entity::entity_types::EntityTypeCache;
use gridmap::grid::{Gridmap, GridmapCache};
use itertools::Itertools;
use networking::stamp::{step_tickrate_stamp, TickRateStamp};
use physics::{
    cache::{Cache, PhysicsCache},
    correction_mode::CorrectionResults,
    entity::{RigidBodies, SFRigidBody},
    sync::{CorrectionServerRigidBodyLink, SimulationStorage},
};
use resources::{
    correction::{CorrectionServerSet, IsCorrecting, StartCorrection},
    modes::is_server,
    physics::{PhysicsSet, PriorityPhysicsCache, PriorityUpdate},
    sets::MainSet,
};

use crate::{start_app, Mode};

/// Creates a headless app instance in correction mode.
pub struct CorrectionPlugin;
impl Plugin for CorrectionPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_systems(
                FixedUpdate,
                (
                    start_correction
                        .after(MainSet::PostUpdate)
                        .before(receive_correction_server_messages)
                        .after(PhysicsSet::Cache),
                    receive_correction_server_messages.in_set(MainSet::PostPhysics),
                    apply_correction_results
                        .after(receive_correction_server_messages)
                        .in_set(PhysicsSet::Correct)
                        .in_set(MainSet::PostPhysics),
                ),
            )
            .add_systems(Startup, start_correction_server)
            .init_non_send_resource::<CorrectionServerReceiveMessage>()
            .init_resource::<CorrectionEnabled>();
        }
    }
}

/// Runs on the correction app.
pub struct CorrectionServerPlugin;
impl Plugin for CorrectionServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                init_correction_server.in_set(MainSet::PreUpdate),
                finish_correction.in_set(MainSet::Fin),
                store_tick_data.in_set(MainSet::PostPhysics),
                apply_controller_caches
                    .in_set(MainSet::PreUpdate)
                    .after(CorrectionServerSet::TriggerSync),
            ),
        )
        .add_systems(
            Update,
            server_start_correcting
                .before(MainSet::PreUpdate)
                .before(step_tickrate_stamp)
                .in_set(CorrectionServerSet::TriggerSync),
        )
        .init_resource::<StartCorrection>()
        .init_resource::<IsCorrecting>();
    }
}
pub enum ConsoleCommandsSet {
    Input,
    Finalize,
}

const IDLE_LOOP_TIME: f64 = 100000.;

pub(crate) fn finish_correction(
    stamp: Res<TickRateStamp>,
    mut correcting: ResMut<IsCorrecting>,
    start: Res<StartCorrection>,
    send: Res<CorrectionServerSendMessage>,
    mut fixed: ResMut<Time<Fixed>>,
    mut storage: ResMut<SimulationStorage>,
    link: Res<CorrectionServerRigidBodyLink>,
) {
    if correcting.0 && stamp.large == start.last_tick {
        correcting.0 = false;

        let mut new_storage = storage.0.clone();

        for ncache in new_storage.cache.iter_mut() {
            for (_, cache) in ncache.1 {
                let mut found = false;
                for (client, sims) in link.map.iter() {
                    for e in sims.iter() {
                        if *e == cache.entity {
                            cache.entity = *client;
                            found = true;
                        }
                    }
                }
                if !found {
                    warn!(
                        "Couldnt find CorrectionServerRigidBodyLink link: {:?}, len: {}",
                        cache.entity,
                        link.map.len()
                    );
                }
            }
        }

        for tick in new_storage.cache.keys().sorted() {
            if *tick > start.last_tick {
                warn!(
                    "SimulationStorage contains tick {} greater than last tick {}",
                    tick, start.last_tick
                );
            } else if *tick <= start.start_tick {
                warn!(
                    "SimulationStorage contains tick {} less than start tick {}",
                    tick, start.start_tick
                );
            }
        }

        match send
            .sender
            .send(CorrectionServerMessage::Results(CorrectionResults {
                data: new_storage,
            })) {
            Ok(_) => {
                fixed.set_timestep_seconds(IDLE_LOOP_TIME);
                storage.0 = PhysicsCache::default();
            }
            Err(_) => {
                warn!("Couldnt send finish correction message.");
            }
        }
    }
}

/// Correction server system.
/// Messages get created when client spawns in an entity and when new peer input has been received.
pub(crate) fn server_start_correcting(
    queued_message_reciever: NonSend<CorrectionServerReceiveMessage>,
    mut cache: ResMut<PhysicsCache>,
    mut fixed: ResMut<Time<Fixed>>,
    mut correction: ResMut<StartCorrection>,
    mut gridmap: ResMut<Gridmap>,
    mut stamp: ResMut<TickRateStamp>,
    link: Res<CorrectionServerRigidBodyLink>,
    mut controller_cache: ResMut<ControllerCache>,
    mut look_cache: ResMut<LookTransformCache>,
    mut priority: ResMut<PriorityPhysicsCache>,
    query: Query<Entity, With<RigidBody>>,
    mut correcting: ResMut<IsCorrecting>,
    mut entity_type_cache: ResMut<EntityTypeCache>,
) {
    match &queued_message_reciever.receiver_option {
        Some(receiver) => loop {
            let queued_message_result = receiver.try_recv();

            match queued_message_result {
                Ok(incoming_message) => match incoming_message {
                    ClientCorrectionMessage::StartCorrecting(
                        start_correction_data,
                        new_cache,
                        gridmap_cache,
                        controller_cachec,
                        look_cachec,
                        priorityc,
                        type_cache,
                    ) => {
                        let mut fixed_cache = new_cache.clone();

                        for t in fixed_cache.cache.iter_mut() {
                            for (_, cache) in t.1.iter_mut() {
                                let mut found: bool = false;
                                for (client, sims) in link.map.iter() {
                                    for sim in sims.iter() {
                                        if *client == cache.entity {
                                            cache.entity = *sim;
                                            found = true;

                                            break;
                                        }
                                    }
                                }
                                if !found {
                                    //warn!("Cache link not found.");
                                }
                            }
                        }
                        let mut new_pcache = HashMap::new();
                        for t in priorityc.cache.iter() {
                            let mut new_tick_map = HashMap::new();
                            for (pentity, update) in t.1.iter() {
                                let mut found: bool = false;
                                for (client, sims) in link.map.iter() {
                                    if client == pentity {
                                        let mut f = None;

                                        for sim in sims.iter() {
                                            match query.get(*sim) {
                                                Ok(_) => {
                                                    f = Some(*sim);
                                                    break;
                                                }
                                                Err(_) => {}
                                            }
                                        }
                                        match f {
                                            Some(sim) => {
                                                let new_update = update.clone();
                                                match new_update {
                                                    PriorityUpdate::SmallCache(mut small) => {
                                                        small.entity = sim;
                                                        new_tick_map.insert(
                                                            sim,
                                                            PriorityUpdate::SmallCache(small),
                                                        );
                                                    }
                                                    _ => {
                                                        new_tick_map.insert(sim, new_update);
                                                    }
                                                }
                                                found = true;
                                            }
                                            None => {
                                                //warn!("Nothing found.");
                                            }
                                        }
                                        break;
                                    }
                                }
                                if !found {
                                    //warn!("pCache link not found.");
                                }
                            }
                            new_pcache.insert(*t.0, new_tick_map);
                        }
                        *cache = fixed_cache;
                        fixed.set_timestep_seconds(0.000000001);
                        *correction = start_correction_data.clone();
                        gridmap.updates_cache = gridmap_cache;

                        *controller_cache = controller_cachec;
                        *look_cache = look_cachec;
                        // -1 because this system happens before step_tickrate_stamp system.
                        // -1 because constructing a physics scene from cache needs an entire frame for itself to initialize.
                        *stamp = TickRateStamp::new(start_correction_data.start_tick - 2);
                        priority.cache = new_pcache;
                        correcting.0 = true;
                        *entity_type_cache = type_cache;
                    }
                },
                Err(_) => {
                    break;
                }
            }
        },
        None => {}
    }
}

pub(crate) fn init_correction_server(mut first: Local<bool>, mut fixed: ResMut<Time<Fixed>>) {
    if !*first {
        *first = true;
    } else {
        return;
    }

    fixed.set_timestep_seconds(IDLE_LOOP_TIME);
}

#[derive(Default)]
pub struct CorrectionServerReceiveMessage {
    pub receiver_option: Option<Receiver<ClientCorrectionMessage>>,
}
#[derive(Resource)]
pub struct CorrectionServerSendMessage {
    pub sender: SyncSender<CorrectionServerMessage>,
}
#[derive(Resource)]
pub struct CorrectionServerData {
    pub message_sender: SyncSender<ClientCorrectionMessage>,
    pub app_handle: JoinHandle<()>,
}
pub struct CorrectionServerMessageReceiver {
    pub receiver: Receiver<CorrectionServerMessage>,
}
pub enum ClientCorrectionMessage {
    StartCorrecting(
        StartCorrection,
        PhysicsCache,
        GridmapCache,
        ControllerCache,
        LookTransformCache,
        PriorityPhysicsCache,
        EntityTypeCache,
    ),
}
pub enum CorrectionServerMessage {
    Results(CorrectionResults),
}

/// Spin up another client app instance in correction mode.
pub(crate) fn start_correction_server(world: &mut World) {
    let (tx, rx) = mpsc::sync_channel(64);
    let message_receiver = CorrectionServerReceiveMessage {
        receiver_option: Some(rx),
    };

    let (tx2, rx2) = mpsc::sync_channel(64);

    let builder = std::thread::Builder::new().name("Correction Server".to_string());

    match builder.spawn(move || start_app(Mode::Correction(message_receiver, tx2))) {
        Ok(app) => {
            info!("Physics correction server started.");
            world.insert_resource(CorrectionServerData {
                message_sender: tx,
                app_handle: app,
            });
            world.insert_non_send_resource(CorrectionServerMessageReceiver { receiver: rx2 });
        }
        Err(_) => {
            error!("Couldnt spawn correction server thread.");
        }
    }
}
#[derive(Default, Resource)]
pub struct CorrectionEnabled(pub bool);

pub(crate) fn start_correction(
    mut events: EventReader<StartCorrection>,
    physics_cache: Res<PhysicsCache>,
    //mut iterative_i: ResMut<CorrectionResource>,
    correction_server: Res<CorrectionServerData>,
    grid: Res<Gridmap>,
    mut enabled: ResMut<CorrectionEnabled>,
    look_cache: Res<LookTransformCache>,
    controller_cache: Res<ControllerCache>,
    priority: Res<PriorityPhysicsCache>,
    stamp: Res<TickRateStamp>,
    mut previous_lowest_start: Local<u64>,
    entity_type_cache: Res<EntityTypeCache>,
) {
    let mut lowest_start = 0;
    let mut highest_end = 1;
    let mut one_event = false;
    let mut first_event = true;
    for event in events.read() {
        if first_event {
            lowest_start = event.start_tick;
            highest_end = event.last_tick;
        } else {
            if event.start_tick < lowest_start {
                lowest_start = event.start_tick
            }
            if event.last_tick > highest_end {
                highest_end = event.last_tick
            }
        }
        one_event = true;
        first_event = false;
    }
    if !one_event {
        return;
    }
    if highest_end != stamp.large {
        warn!("StartCorrection received last tick that is not equal to stamp.large");
    }
    if lowest_start < *previous_lowest_start {
        info!("StartCorrection received start tick that is less than previous lowest start.");
    } else {
        *previous_lowest_start = lowest_start;
    }
    if lowest_start == highest_end {
        warn!("StartCorrection lowest and highest are equal.");
    }
    /*info!(
        "Start correction {}-{} at tick {}",
        lowest_start, highest_end, stamp.large
    );*/
    match correction_server
        .message_sender
        .send(ClientCorrectionMessage::StartCorrecting(
            StartCorrection {
                start_tick: lowest_start,
                last_tick: highest_end,
            },
            physics_cache.clone(),
            grid.updates_cache.clone(),
            controller_cache.clone(),
            look_cache.clone(),
            priority.clone(),
            entity_type_cache.clone(),
        )) {
        Ok(_) => {
            enabled.0 = true;
        }
        Err(rr) => {
            warn!("Couldnt start correction: {}", rr);
        }
    }
}

pub(crate) fn receive_correction_server_messages(
    receiver: NonSend<CorrectionServerMessageReceiver>,
    mut send: EventWriter<CorrectionResults>,
    mut waiting: ResMut<CorrectionEnabled>,
    rigidbodies: Res<RigidBodies>,
) {
    if waiting.0 {
        match receiver.receiver.recv() {
            Ok(correction_server_message) => match correction_server_message {
                CorrectionServerMessage::Results(mut results) => {
                    for t in results.data.cache.iter_mut() {
                        for (_, cache) in t.1 {
                            match rigidbodies.get_entity_rigidbody(&cache.entity) {
                                Some(rb_entity) => {
                                    cache.rb_entity = *rb_entity;
                                }
                                None => {
                                    //warn!("Couldnt get entity rigidbody.");
                                }
                            }
                        }
                    }
                    send.send(results);
                    waiting.0 = false;
                }
            },
            Err(rr) => {
                warn!("recv() error: {:?}", rr);
            }
        }
    }
}

/// Correction server system.
pub(crate) fn store_tick_data(
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
    mut storage: ResMut<SimulationStorage>,
    stamp: Res<TickRateStamp>,
    correcting: Res<IsCorrecting>,
    correction: Res<StartCorrection>,
    type_cache: Res<EntityTypeCache>,
    link: Res<CorrectionServerRigidBodyLink>,
) {
    if !correcting.0 || stamp.large <= correction.start_tick {
        return;
    }
    storage.0.cache.insert(stamp.large, HashMap::new());

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
        let tick_cache = storage.0.cache.get_mut(&stamp.large).unwrap();
        let entity_type;
        match link.get_client(&rb_entity) {
            Some(clink) => match type_cache.map.get(clink) {
                Some(t) => {
                    entity_type = t.clone();
                }
                None => {
                    warn!("No typecache match. {:?}", clink);
                    continue;
                }
            },
            None => {
                warn!("No client link found.");
                continue;
            }
        }
        tick_cache.insert(
            rb_entity,
            Cache {
                entity: rb_entity,
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
                entity_type: entity_type,
                spawn_frame: false,
            },
        );
    }
}
/// Runs on client.
pub(crate) fn apply_correction_results(
    mut events: EventReader<CorrectionResults>,
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
    stamp: Res<TickRateStamp>,
    mut cache: ResMut<PhysicsCache>,
    mut commands: Commands,
) {
    for event in events.read() {
        for (tick, tick_cache) in event.data.cache.iter() {
            match cache.cache.get_mut(tick) {
                Some(modern) => {
                    for (_, cache) in tick_cache {
                        modern.insert(cache.entity, cache.clone());
                    }
                }
                None => {
                    cache.cache.insert(*tick, tick_cache.clone());
                }
            }
        }

        match event.data.cache.get(&stamp.large) {
            Some(cache_vec) => {
                for (_, cache) in cache_vec {
                    match query.get_mut(cache.rb_entity) {
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
                                commands.entity(cache.rb_entity).insert(Sleeping);
                            } else {
                                commands.entity(cache.rb_entity).remove::<Sleeping>();
                            }
                        }
                        Err(_rr) => {
                            //warn!("Couldnt find entity: {:?}", cache.rb_entity);
                        }
                    }
                }
            }
            None => {
                warn!(
                    "Correction results did not contain current tick: {}",
                    stamp.large
                );
            }
        }
    }
}

pub(crate) fn apply_controller_caches(
    controller_cache: Res<ControllerCache>,
    look_cache: Res<LookTransformCache>,
    mut query: Query<(Entity, &mut LookTransform, &mut ControllerInput)>,
    stamp: Res<TickRateStamp>,
    link: Res<CorrectionServerRigidBodyLink>,
    correcting: Res<IsCorrecting>,
    start: Res<StartCorrection>,
) {
    if !correcting.0 {
        return;
    }
    if stamp.large > start.start_tick {
        return;
    }
    for (entity, mut look, mut controller) in query.iter_mut() {
        let client_entity;
        match link.get_client(&entity) {
            Some(client) => {
                client_entity = *client;
            }
            None => {
                warn!("Couldnt find link of query. continueing. {:?}", entity);
                continue;
            }
        }

        let mut look_t = None;

        let mut controller_t = None;

        match controller_cache.cache.get(&client_entity) {
            Some(tick_cache) => {
                for tick in tick_cache.keys().sorted().rev() {
                    if tick > &stamp.large {
                        continue;
                    }
                    controller_t = Some(tick_cache.get(tick).unwrap());
                    break;
                }
            }
            None => {}
        }
        match look_cache.cache.get(&client_entity) {
            Some(tick_cache) => {
                for tick in tick_cache.keys().sorted().rev() {
                    if tick > &stamp.large {
                        continue;
                    }
                    look_t = Some(tick_cache.get(tick).unwrap());
                    break;
                }
            }
            None => {}
        }

        match controller_t {
            Some(controller_t) => {
                *controller = controller_t.clone();
            }
            None => {
                //warn!("No available controller cache.");
            }
        }
        match look_t {
            Some(look_t) => {
                if look.target != look_t.target {
                    /*info!(
                        "Correction tick {} applying look transform {:?}",
                        stamp.large, look.target
                    );*/
                }

                *look = look_t.clone();
            }
            None => {
                //warn!("No available look cache.");
            }
        }
    }
}
