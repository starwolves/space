use bevy::log::info;
use bevy::{
    prelude::{EventReader, EventWriter, Local, Res, ResMut, Resource},
    time::{Fixed, Time},
};

use bevy_xpbd_3d::prelude::{Physics, PhysicsTime};
use networking::{
    client::{
        IncomingReliableServerMessage, NetworkingClientMessage, OutgoingReliableClientMessage,
    },
    server::{AdjustSync, NetworkingServerMessage},
    stamp::{PauseTickStep, TickRateStamp},
};
use resources::core::TickRate;
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
            fixed_time.set_timestep_seconds(1. / TickRate::default().bevy_rate as f64);
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
                        (1. / TickRate::default().bevy_rate as f64) / (delta.abs() + 1) as f64,
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
