use std::{
    collections::{BTreeMap, HashMap},
    sync::mpsc::{self, Receiver, SyncSender},
    thread::JoinHandle,
    time::Duration,
};

use bevy::{
    app::{PostStartup, Startup, SubApp},
    ecs::{entity::Entity, schedule::SystemSet, system::Commands},
    log::{error, warn},
};
use bevy::{
    ecs::{query::With, system::Query},
    log::info,
    transform::components::Transform,
};
use bevy::{
    prelude::{App, EventReader, IntoSystemConfigs, Local, Plugin, Res, ResMut, Resource, World},
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
use networking::stamp::TickRateStamp;
use physics::{
    cache::{Cache, PhysicsCache},
    correction_mode::CorrectionResults,
    entity::{RigidBodies, SFRigidBody},
    plugin::PhysicsStepSet,
    sync::{CorrectionEnabled, CorrectionServerRigidBodyLink, SimulationStorage},
};
use resources::{
    correction::{
        CorrectionServerSet, IsCorrecting, ObtainedSynchronousSyncData, StartCorrection,
        SynchronousCorrection, SynchronousCorrectionOnGoing,
    },
    modes::is_correction_mode,
    ordering::{Fin, PostUpdate, PreUpdate},
    physics::{PhysicsSet, PriorityPhysicsCache, PriorityUpdate},
};

use crate::{init_app, step_game_schedules};

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct Correction;

pub struct CorrectionPlugin;
impl Plugin for CorrectionPlugin {
    fn build(&self, app: &mut App) {
        if !is_correction_mode(app) {
            app.add_systems(
                PostUpdate,
                (
                    start_correction.before(message_correction_server),
                    message_correction_server,
                    apply_correction_results
                        .after(message_correction_server)
                        .in_set(PhysicsSet::Correct),
                )
                    .in_set(Correction),
            )
            .add_systems(PostStartup, start_synchronous_correction_server);
        }
        app.init_resource::<StartCorrectingMessage>();
    }
}
#[derive(Resource, Default)]
struct FirsCorrectionTick(pub bool);

/// Runs on the correction app.
pub struct CorrectionServerPlugin;
impl Plugin for CorrectionServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (store_tick_data.after(PhysicsStepSet),))
            .add_systems(
                PreUpdate,
                (
                    init_correction_server,
                    apply_controller_caches.after(CorrectionServerSet::TriggerSync),
                ),
            )
            .add_systems(Fin, (finish_correction,))
            .init_resource::<StartCorrection>()
            .init_resource::<IsCorrecting>()
            .init_resource::<FirsCorrectionTick>();
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
    mut results: ResMut<CorrectionResults>,
    mut storage: ResMut<SimulationStorage>,
    mut link: ResMut<CorrectionServerRigidBodyLink>,
    synchronous_correction: Res<SynchronousCorrection>,
    sender: Res<CorrectionResultsSender>,
) {
    if correcting.0 && stamp.tick == start.last_tick {
        correcting.0 = false;

        let new_storage = storage.0.clone();

        for tick in new_storage.cache.keys() {
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

        results.data = new_storage;
        storage.0 = PhysicsCache::default();
        if synchronous_correction.0 {
            match sender.tx.as_ref().unwrap().send(results.clone()) {
                Ok(_) => {}
                Err(rr) => {
                    warn!("Couldnt send message to main thread {}", rr);
                }
            }
        }
        link.clean_despawned();
    }
}

/// Correction app exclusive system.
pub(crate) fn server_start_correcting(world: &mut World) {
    let synchronous_mode = world.resource::<SynchronousCorrection>().0;
    let start_data;
    if !synchronous_mode {
        let mut first = false;
        (|| {
            let mut f = world.resource_mut::<FirsCorrectionTick>();
            first = f.0;
            f.0 = true;
        })();

        // First tick.
        if !first {
            world.run_schedule(Startup);
        }
        start_data = world
            .get_resource::<StartCorrectingMessage>()
            .unwrap()
            .clone();

        if !start_data.correcting {
            return;
        }
    } else {
        let receiver = world.non_send_resource::<CorrectionServerMessageReceiver>();
        match receiver.receiver.recv() {
            Ok(d) => {
                start_data = d;
            }
            Err(rr) => {
                warn!("failed to receive main thread message {}", rr);
                return;
            }
        }
    }

    let mut fixed_cache = start_data.physics_cache.clone();
    let link = || -> CorrectionServerRigidBodyLink {
        let link = world
            .get_resource::<CorrectionServerRigidBodyLink>()
            .unwrap();
        for t in fixed_cache.cache.iter_mut() {
            for (_, cache) in t.1.iter_mut() {
                let mut found: bool = false;
                match link.map.get(&cache.entity) {
                    Some(sims) => {
                        for sim in &sims.known_links {
                            cache.entity = *sim;
                            found = true;

                            break;
                        }
                    }
                    None => {}
                }
                if !found {
                    //warn!("Cache link not found.");
                }
            }
        }
        link.clone()
    }();

    let mut query = world.query_filtered::<Entity, With<RigidBody>>();

    let mut new_pcache = BTreeMap::new();
    for t in start_data.priority_physics_cache.cache.iter() {
        let mut new_tick_map = HashMap::new();
        for (pentity, update) in t.1.iter() {
            let mut found: bool = false;
            for (client, sims) in link.map.iter() {
                if client == pentity {
                    let mut f = None;
                    match sims.active_link {
                        Some(sim) => match query.get(world, sim) {
                            Ok(_) => {
                                f = Some(sim);
                            }
                            Err(_) => {}
                        },
                        None => {}
                    }
                    match f {
                        Some(sim) => {
                            let new_update = update.clone();
                            match new_update {
                                PriorityUpdate::SmallCache(mut small) => {
                                    small.entity = sim;
                                    new_tick_map.insert(sim, PriorityUpdate::SmallCache(small));
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
    (|| {
        let mut cache = world.resource_mut::<PhysicsCache>();
        *cache = fixed_cache;
    })();

    (|| {
        let mut correction = world.resource_mut::<StartCorrection>();
        *correction = start_data.start.clone();
    })();

    (|| {
        let mut stamp = world.resource_mut::<TickRateStamp>();
        // -1 because this system happens before step_tickrate_stamp system.
        // -1 because constructing a physics scene from cache needs an entire frame for itself to initialize.
        *stamp = TickRateStamp::new(start_data.start.start_tick - 2);
    })();

    (|| {
        let mut controller_cache = world.resource_mut::<ControllerCache>();
        *controller_cache = start_data.controller_cache;
    })();

    (|| {
        let mut look_cache = world.resource_mut::<LookTransformCache>();
        *look_cache = start_data.look_transform_cache;
    })();

    (|| {
        let mut priority = world.resource_mut::<PriorityPhysicsCache>();

        priority.cache = new_pcache;
    })();

    (|| {
        let mut correcting = world.resource_mut::<IsCorrecting>();

        correcting.0 = true;
    })();

    (|| {
        let mut entity_type_cache = world.resource_mut::<EntityTypeCache>();
        *entity_type_cache = start_data.entity_type_cache;
    })();

    for _ in start_data.start.start_tick - 1..start_data.start.last_tick + 1 {
        step_game_schedules(world);
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

#[derive(Resource, Default, Clone)]
pub struct StartCorrectingMessage {
    pub start: StartCorrection,
    pub physics_cache: PhysicsCache,
    pub controller_cache: ControllerCache,
    pub look_transform_cache: LookTransformCache,
    pub priority_physics_cache: PriorityPhysicsCache,
    pub entity_type_cache: EntityTypeCache,
    pub correcting: bool,
}
pub enum CorrectionAppMessage {
    Results(CorrectionResults),
}

pub struct CorrectionApp {
    pub app: SubApp,
}

pub(crate) fn start_correction(
    mut events: EventReader<StartCorrection>,
    physics_cache: Res<PhysicsCache>,
    //mut iterative_i: ResMut<CorrectionResource>,
    mut enabled: ResMut<CorrectionEnabled>,
    look_cache: Res<LookTransformCache>,
    controller_cache: Res<ControllerCache>,
    priority: Res<PriorityPhysicsCache>,
    stamp: Res<TickRateStamp>,
    mut previous_lowest_start: Local<u32>,
    entity_type_cache: Res<EntityTypeCache>,
    mut start_message: ResMut<StartCorrectingMessage>,
    mut ongoing: ResMut<SynchronousCorrectionOnGoing>,
    synchronous_correction: Res<SynchronousCorrection>,
) {
    if synchronous_correction.0 {
        ongoing.step();
    }

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
    if highest_end != stamp.tick {
        warn!("StartCorrection received last tick that is not equal to stamp.tick");
    }
    if lowest_start < *previous_lowest_start {
        info!("StartCorrection received start tick that is less than previous lowest start.");
    } else {
        *previous_lowest_start = lowest_start;
    }
    if lowest_start == highest_end {
        warn!("StartCorrection lowest and highest are equal.");
    }
    if synchronous_correction.0 {
        highest_end += 1;
    }
    /*info!(
        "Start correction {}-{} at tick {}",
        lowest_start, highest_end, stamp.tick
    );*/
    *start_message = StartCorrectingMessage {
        start: StartCorrection {
            start_tick: lowest_start,
            last_tick: highest_end,
        },
        physics_cache: physics_cache.clone(),
        controller_cache: controller_cache.clone(),
        look_transform_cache: look_cache.clone(),
        priority_physics_cache: priority.clone(),
        entity_type_cache: entity_type_cache.clone(),
        correcting: true,
    };
    enabled.0 = true;
    if synchronous_correction.0 {
        ongoing.0.push(false);
    }
}

pub(crate) fn message_correction_server(world: &mut World) {
    let synchronous_mode = world.resource::<SynchronousCorrection>().0;
    if !synchronous_mode {
        if !world.resource::<CorrectionEnabled>().0 {
            return;
        }
    }
    let rigidbodies = world.resource::<RigidBodies>().clone();

    if synchronous_mode {
        if world
            .resource::<SynchronousCorrectionOnGoing>()
            .receive_ready()
        {
            let correction_results_receiver =
                world.non_send_resource::<ReceiveMessageFromCorrectionServer>();
            match &correction_results_receiver.receiver_option {
                Some(rcv) => match rcv.recv_timeout(Duration::from_secs_f32(5.)) {
                    Ok(data) => {
                        let mut old_data = world.resource_mut::<CorrectionResults>();
                        *old_data = data;
                        world
                            .resource_mut::<SynchronousCorrectionOnGoing>()
                            .0
                            .remove(0);
                        world.resource_mut::<ObtainedSynchronousSyncData>().0 = true;
                    }
                    Err(rr) => {
                        warn!("Eror receiving correction results: {}", rr);
                    }
                },
                None => {
                    warn!("No receiver found.");
                }
            }
        }
        if world
            .resource::<SynchronousCorrectionOnGoing>()
            .send_ready()
        {
            let start_correction_messenger = world.resource::<StartCorrectionSender>();
            let start_data = world.resource::<StartCorrectingMessage>();
            match start_correction_messenger
                .message_sender
                .send(start_data.clone())
            {
                Ok(_) => {}
                Err(_) => {
                    warn!("Failed to send message to correction thread.");
                }
            }
        }
    } else {
        let mut correction = world.remove_non_send_resource::<CorrectionApp>().unwrap();

        correction.app.extract(world);
        correction.app.run();
        correction.app.extract(world);
        world.insert_non_send_resource(correction);
    }

    let mut results = world.resource_mut::<CorrectionResults>();

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
    physics_cache: Res<PhysicsCache>,
) {
    if !correcting.0 || stamp.tick <= correction.start_tick {
        return;
    }
    storage.0.cache.insert(stamp.tick, HashMap::new());

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
        let tick_cache = storage.0.cache.get_mut(&stamp.tick).unwrap();
        let entity_type;
        let client_entity;
        match link.get_client(&rb_entity) {
            Some(clink) => {
                client_entity = *clink;
                match type_cache.map.get(clink) {
                    Some(t) => {
                        entity_type = t.clone();
                    }
                    None => {
                        warn!("No typecache match. {:?}", clink);
                        continue;
                    }
                }
            }
            None => {
                warn!("No client link found.");
                continue;
            }
        }
        match physics_cache.cache.get(&stamp.tick) {
            Some(c) => match c.get(&rb_entity) {
                Some(x) => {
                    if x.spawn_frame {
                        continue;
                    }
                }
                // This match may be unnecessary.
                None => match c.get(&client_entity) {
                    Some(x) => {
                        if x.spawn_frame {
                            continue;
                        }
                    }
                    None => {}
                },
            },
            None => {}
        }
        tick_cache.insert(
            rb_entity,
            Cache {
                entity: client_entity,
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
    correction_results: Res<CorrectionResults>,
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
    mut waiting: ResMut<CorrectionEnabled>,
    synchronous_correction: Res<SynchronousCorrection>,
    mut obtained: ResMut<ObtainedSynchronousSyncData>,
) {
    if synchronous_correction.0 {
        if obtained.0 {
            obtained.0 = false;
        } else {
            return;
        }
    } else {
        if !waiting.0 {
            return;
        }
    }
    waiting.0 = false;

    for (tick, tick_cache) in correction_results.data.cache.iter() {
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

    match correction_results.data.cache.get(&stamp.tick) {
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
                        warn!(
                            "apply_correction_results Couldnt find entity: {:?}",
                            cache.rb_entity
                        );
                    }
                }
            }
        }
        None => {
            let mut d = vec![];
            for tick in correction_results.data.cache.keys().rev() {
                d.push(*tick);
            }
            warn!(
                "Correction results did not contain current tick: {} {:?}",
                stamp.tick, d
            );
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
    if stamp.tick <= start.start_tick {
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
                for tick in tick_cache.keys().rev() {
                    if tick > &stamp.tick {
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
                for tick in tick_cache.keys().rev() {
                    if tick > &stamp.tick {
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
                /*info!(
                    "Setting {:?} {:?} at {} start {:?}",
                    client_entity, controller, stamp.tick, start
                );*/
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
                        stamp.tick, look.target
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
#[derive(Default)]
pub struct ReceiveMessageFromCorrectionServer {
    pub receiver_option: Option<Receiver<CorrectionResults>>,
}
#[derive(Resource)]
pub struct StartCorrectionSender {
    pub message_sender: SyncSender<StartCorrectingMessage>,
    pub app_handle: JoinHandle<()>,
}
pub struct CorrectionServerMessageReceiver {
    pub receiver: Receiver<StartCorrectingMessage>,
}
pub(crate) struct CorrectionMessengers {
    pub rx: CorrectionServerMessageReceiver,
    pub tx: SyncSender<CorrectionResults>,
}
#[derive(Resource)]
pub(crate) struct CorrectionResultsSender {
    pub tx: Option<SyncSender<CorrectionResults>>,
}
/// Spin up a parallel correction instance.
pub(crate) fn start_synchronous_correction_server(world: &mut World) {
    let synchronous_mode = world.resource::<SynchronousCorrection>().0;
    if !synchronous_mode {
        return;
    }
    let (tx, rx) = mpsc::sync_channel(64);
    let message_receiver = ReceiveMessageFromCorrectionServer {
        receiver_option: Some(rx),
    };

    let (tx2, rx2) = mpsc::sync_channel(64);

    let builder = std::thread::Builder::new().name("Correction Server".to_string());
    match builder.spawn(move || {
        init_app(Some(CorrectionMessengers {
            rx: CorrectionServerMessageReceiver { receiver: rx2 },
            tx: tx,
        }))
    }) {
        Ok(app) => {
            info!("Started correction server.");
            world.insert_resource(StartCorrectionSender {
                message_sender: tx2,
                app_handle: app,
            });
            world.insert_non_send_resource(message_receiver);
        }
        Err(_) => {
            error!("Couldnt spawn correction server thread.");
        }
    }
}
