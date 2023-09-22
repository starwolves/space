use std::time::Duration;

use bevy::prelude::{info, EventReader, EventWriter, FixedTime, Local, ResMut, Resource};
use bevy_xpbd_3d::prelude::PhysicsLoop;
use networking::{
    client::{
        IncomingReliableServerMessage, NetworkingClientMessage, OutgoingReliableClientMessage,
    },
    server::{AdjustSync, NetworkingServerMessage},
    stamp::PauseTickStep,
};
use resources::core::TickRate;
#[derive(Resource, Default)]
pub(crate) struct FastForwarding {
    pub forwarding: bool,
    pub advance: u8,
    pub i: u8,
}

pub const DEBUG_FAST_FORWARD: bool = false;

pub(crate) fn sync_loop(
    mut net: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut physics_loop: ResMut<PhysicsLoop>,
    mut paused: Local<(i8, i8)>,
    mut sync_queue: Local<Vec<AdjustSync>>,
    mut out: EventWriter<OutgoingReliableClientMessage<NetworkingClientMessage>>,
    mut fixed_time: ResMut<FixedTime>,
    mut fast_forwarding: ResMut<FastForwarding>,
    mut p: ResMut<PauseTickStep>,
) {
    if physics_loop.paused {
        paused.1 += 1;
        if paused.1 >= paused.0 {
            physics_loop.resume();
            out.send(OutgoingReliableClientMessage {
                message: NetworkingClientMessage::SyncConfirmation,
            });
        }
    } else if fast_forwarding.forwarding {
        fast_forwarding.i += 1;
        if fast_forwarding.i >= fast_forwarding.advance {
            fast_forwarding.forwarding = false;
            fixed_time.period = Duration::from_secs_f32(1. / TickRate::default().bevy_rate as f32);
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

    for message in net.iter() {
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
            if !physics_loop.paused {
                if adjustment.advance > 0 {
                    paused.0 = adjustment.advance;
                    paused.1 = 0;
                    physics_loop.pause();
                    if process_queue {
                        erase_queue = true;
                        info!("Pause {} ticks (from queue)", paused.0);
                    } else {
                        info!("Pause {} ticks", paused.0);
                    }
                } else {
                    info!("Fast-forward {} ticks", adjustment.advance.abs());

                    fixed_time.period = Duration::from_secs_f32(
                        (1. / TickRate::default().bevy_rate as f32)
                            / adjustment.advance.abs() as f32,
                    );
                    fast_forwarding.forwarding = true;
                    fast_forwarding.i = 0;
                    fast_forwarding.advance = adjustment.advance.abs() as u8;
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
