use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    thread::JoinHandle,
};

use bevy::{ecs::entity::Entity, log::warn};
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
use controller::input::RecordedControllerInput;
use gridmap::grid::{Gridmap, GridmapCache};
use networking::stamp::{step_tickrate_stamp, TickRateStamp};
use physics::{
    cache::{Cache, PhysicsCache, PhysicsSet},
    correction_mode::CorrectionResults,
    entity::SFRigidBody,
};
use resources::{
    correction::{CorrectionServerSet, CorrectionSet, StartCorrection, SyncWorld},
    modes::is_server,
    sets::MainSet,
};

use crate::{start_app, AppMode};

/// Creates a headless app instance in correction mode.
pub struct CorrectionPlugin;
impl Plugin for CorrectionPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_systems(
                FixedUpdate,
                (
                    start_correction
                        .in_set(MainSet::Update)
                        .after(CorrectionSet::Start),
                    receive_correction_server_messages.after(MainSet::PostUpdate),
                    apply_correction_results
                        .after(receive_correction_server_messages)
                        .before(PhysicsSet::Cache)
                        .in_set(PhysicsSet::Correct),
                ),
            )
            .add_systems(Startup, start_correction_server)
            .init_non_send_resource::<CorrectionServerReceiveMessage>()
            .init_resource::<CorrectionEnabled>();
        }
    }
}

/// Runs on the app if in correction mode.
pub struct CorrectionServerPlugin;
impl Plugin for CorrectionServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                init_correction_server.in_set(MainSet::PreUpdate),
                finish_correction.in_set(MainSet::PostUpdate),
                store_tick_data.in_set(MainSet::Update),
                clear_sync_world.in_set(MainSet::PostUpdate),
            ),
        )
        .add_systems(
            Update,
            server_start_correcting
                .before(MainSet::PreUpdate)
                .in_set(CorrectionServerSet::TriggerSync)
                .after(step_tickrate_stamp),
        )
        .init_resource::<StartCorrection>()
        .init_resource::<IsCorrecting>()
        .init_resource::<SyncWorld>()
        .init_resource::<SimulationStorage>();
    }
}
#[derive(Resource, Default)]
pub struct IsCorrecting(bool);

pub enum ConsoleCommandsSet {
    Input,
    Finalize,
}

pub(crate) fn finish_correction(
    stamp: Res<TickRateStamp>,
    mut correcting: ResMut<IsCorrecting>,
    start: Res<StartCorrection>,
    send: Res<CorrectionServerSendMessage>,
    mut fixed: ResMut<Time<Fixed>>,
    mut storage: ResMut<SimulationStorage>,
) {
    if correcting.0 && stamp.large >= start.last_tick {
        correcting.0 = false;

        match send
            .sender
            .send(CorrectionServerMessage::Results(CorrectionResults {
                data: storage.0.clone(),
            })) {
            Ok(_) => {
                fixed.set_timestep_seconds(1.);
                storage.0 = PhysicsCache::default();
            }
            Err(_) => {
                warn!("Couldnt send finish correction message.");
            }
        }
    }
}

pub(crate) fn clear_sync_world(mut sync: ResMut<SyncWorld>) {
    if sync.rebuild && !sync.second_tick {
        sync.second_tick = true;
    }
    if !sync.rebuild {
        sync.second_tick = false;
    }

    sync.rebuild = false;
}

/// Correction server system.
/// Messages get created when client spawns in an entity and when new peer input has been received.
pub(crate) fn server_start_correcting(
    queued_message_reciever: NonSend<CorrectionServerReceiveMessage>,
    mut cache: ResMut<PhysicsCache>,
    mut fixed: ResMut<Time<Fixed>>,
    mut correction: ResMut<StartCorrection>,
    mut correcting: ResMut<IsCorrecting>,
    mut input_cache: ResMut<RecordedControllerInput>,
    mut gridmap: ResMut<Gridmap>,
    mut rebuild: ResMut<SyncWorld>,
    mut stamp: ResMut<TickRateStamp>,
) {
    match &queued_message_reciever.receiver_option {
        Some(receiver) => loop {
            let queued_message_result = receiver.try_recv();

            match queued_message_result {
                Ok(incoming_message) => match incoming_message {
                    ClientCorrectionMessage::StartCorrecting(
                        start_correction_data,
                        new_cache,
                        input,
                        gridmap_cache,
                    ) => {
                        *cache = new_cache;
                        fixed.set_timestep_seconds(0.000000001);
                        *correction = start_correction_data.clone();
                        *input_cache = input;
                        correcting.0 = true;
                        gridmap.updates_cache = gridmap_cache;

                        rebuild.rebuild = true;
                        rebuild.second_tick = false;
                        rebuild.sync_to_tick = start_correction_data.start_tick;
                        *stamp = TickRateStamp::new(start_correction_data.start_tick);
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

    fixed.set_timestep_seconds(1.);
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
        RecordedControllerInput,
        GridmapCache,
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

    let app = std::thread::spawn(move || start_app(AppMode::Correction(message_receiver, tx2)));
    info!("Physics correction server started.");
    world.insert_resource(CorrectionServerData {
        message_sender: tx,
        app_handle: app,
    });
    world.insert_non_send_resource(CorrectionServerMessageReceiver { receiver: rx2 });
}
#[derive(Default, Resource)]
pub struct CorrectionEnabled(pub bool);

pub(crate) fn start_correction(
    mut events: EventReader<StartCorrection>,
    input_cache: Res<RecordedControllerInput>,
    physics_cache: Res<PhysicsCache>,
    //mut iterative_i: ResMut<CorrectionResource>,
    correction_server: Res<CorrectionServerData>,
    grid: Res<Gridmap>,
    mut enabled: ResMut<CorrectionEnabled>,
) {
    let mut lowest_start = 0;
    let mut highest_end = 1;
    let mut one_event = false;
    for event in events.read() {
        if !one_event {
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
    }
    if !one_event {
        return;
    }
    match correction_server
        .message_sender
        .send(ClientCorrectionMessage::StartCorrecting(
            StartCorrection {
                start_tick: lowest_start,
                last_tick: highest_end,
            },
            physics_cache.clone(),
            input_cache.clone(),
            grid.updates_cache.clone(),
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
) {
    if waiting.0 {
        match receiver.receiver.recv() {
            Ok(correction_server_message) => match correction_server_message {
                CorrectionServerMessage::Results(results) => {
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
#[derive(Resource, Default)]
pub(crate) struct SimulationStorage(PhysicsCache);

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
                &Sleeping,
                &LockedAxes,
                &CollisionLayers,
            ),
            &Friction,
        ),
        With<SFRigidBody>,
    >,
    mut storage: ResMut<SimulationStorage>,
    stamp: Res<TickRateStamp>,
) {
    storage.0.cache.insert(stamp.large, vec![]);

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
        tick_cache.push(Cache {
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
            sleeping: *sleeping,
            collision_layers: *collision_layers,
            locked_axes: *locked_axes,
            collider_friction: *collider_friction,
        });
    }
}

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
            &mut Sleeping,
            &mut LockedAxes,
            &mut CollisionLayers,
            &mut Friction,
        ),
        With<SFRigidBody>,
    >,
    stamp: Res<TickRateStamp>,
    mut cache: ResMut<PhysicsCache>,
) {
    for event in events.read() {
        *cache = event.data.clone();
        match event.data.cache.get(&stamp.large) {
            Some(cache_vec) => {
                for cache in cache_vec {
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
                            mut sleeping,
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
                            *sleeping = cache.sleeping;
                            *locked_axes = cache.locked_axes;
                            *collision_layers = cache.collision_layers;
                            *friction = cache.collider_friction;
                        }
                        Err(_rr) => {
                            warn!("Couldnt find entity: {:?}", cache.rb_entity);
                        }
                    }
                }
            }
            None => {
                warn!("Couldnt find the right tickrate: {}", stamp.large);
            }
        }
    }
}
