use std::{
    sync::mpsc::{self, Receiver, SyncSender},
    thread::JoinHandle,
};

use bevy::log::info;
use bevy::log::warn;
use bevy::{
    prelude::{
        App, EventReader, EventWriter, FixedUpdate, IntoSystemConfigs, Local, NonSend, Plugin, Res,
        ResMut, Resource, Startup, Update, World,
    },
    time::{Fixed, Time},
};
use controller::input::RecordedControllerInput;
use gridmap::grid::{Gridmap, GridmapCache};
use physics::{
    cache::PhysicsCache,
    correction_mode::{CorrectionResults, StartCorrection},
};
use resources::{modes::is_server, sets::MainSet};

use crate::{start_app, AppMode};

/// Creates a headless app instance in correction mode.
pub struct CorrectionPlugin;
impl Plugin for CorrectionPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_systems(
                FixedUpdate,
                (
                    start_correction.in_set(MainSet::Update),
                    receive_correction_server_messages.in_set(MainSet::PreUpdate),
                ),
            )
            .add_systems(Startup, start_correction_server)
            .init_non_send_resource::<CorrectionServerReceiveMessage>();
        }
    }
}

/// Runs on the app if in correction mode.
pub struct CorrectionServerPlugin;
impl Plugin for CorrectionServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (init_connection_server.in_set(MainSet::PreUpdate),),
        )
        .add_systems(Update, server_start_correcting.before(MainSet::PreUpdate))
        .init_resource::<StartCorrection>()
        .init_resource::<IsCorrecting>();
    }
}
#[derive(Resource, Default)]
pub struct IsCorrecting(bool);

/// Correction server system.
pub(crate) fn server_start_correcting(
    queued_message_reciever: NonSend<CorrectionServerReceiveMessage>,
    mut cache: ResMut<PhysicsCache>,
    mut fixed: ResMut<Time<Fixed>>,
    mut correction: ResMut<StartCorrection>,
    mut correcting: ResMut<IsCorrecting>,
    mut input_cache: ResMut<RecordedControllerInput>,
    mut gridmap: ResMut<Gridmap>,
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
                        fixed.set_timestep_seconds(0.);
                        *correction = start_correction_data;
                        *input_cache = input;
                        correcting.0 = true;
                        gridmap.updates_cache = gridmap_cache;
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

pub(crate) fn init_connection_server(mut first: Local<bool>, mut fixed: ResMut<Time<Fixed>>) {
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

pub(crate) fn start_correction(
    mut events: EventReader<StartCorrection>,
    input_cache: Res<RecordedControllerInput>,
    physics_cache: Res<PhysicsCache>,
    //mut iterative_i: ResMut<CorrectionResource>,
    correction_server: Res<CorrectionServerData>,
    grid: Res<Gridmap>,
) {
    for event in events.read() {
        match correction_server
            .message_sender
            .send(ClientCorrectionMessage::StartCorrecting(
                event.clone(),
                physics_cache.clone(),
                input_cache.clone(),
                grid.updates_cache.clone(),
            )) {
            Ok(_) => {}
            Err(rr) => {
                warn!("Couldnt start correction: {}", rr);
            }
        }
    }
}

pub(crate) fn receive_correction_server_messages(
    receiver: NonSend<CorrectionServerMessageReceiver>,
    mut send: EventWriter<CorrectionResults>,
) {
    loop {
        let queued_message_result = receiver.receiver.try_recv();
        match queued_message_result {
            Ok(correction_server_message) => match correction_server_message {
                CorrectionServerMessage::Results(results) => {
                    send.send(results);
                }
            },
            Err(_) => {
                break;
            }
        }
    }
}
