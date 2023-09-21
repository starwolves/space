use bevy::prelude::{info, EventReader, EventWriter, Local, ResMut};
use bevy_xpbd_3d::prelude::PhysicsLoop;
use networking::{
    client::{
        IncomingReliableServerMessage, NetworkingClientMessage, OutgoingReliableClientMessage,
    },
    server::{AdjustSync, NetworkingServerMessage},
};

pub(crate) fn pause_loop(
    mut net: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut physics_loop: ResMut<PhysicsLoop>,
    mut paused: Local<(i8, i8)>,
    mut sync_queue: Local<Vec<AdjustSync>>,
    mut out: EventWriter<OutgoingReliableClientMessage<NetworkingClientMessage>>,
) {
    if physics_loop.paused {
        paused.1 += 1;
        if paused.1 >= paused.0 {
            physics_loop.resume();
            out.send(OutgoingReliableClientMessage {
                message: NetworkingClientMessage::SyncConfirmation,
            });
        }
    }

    let mut processed = false;
    for adjustment in sync_queue.iter() {
        if adjustment.advance > 0 {
            if !physics_loop.paused {
                paused.0 = adjustment.advance;
                paused.1 = 0;
                physics_loop.pause();
                info!("Pause {} ticks (from queue)", paused.0);
                processed = true;
                break;
            }
        }
    }
    if processed {
        sync_queue.remove(0);
    }

    for message in net.iter() {
        match &message.message {
            NetworkingServerMessage::AdjustSync(adjustment) => {
                if adjustment.advance > 0 {
                    if !physics_loop.paused {
                        paused.0 = adjustment.advance;
                        paused.1 = 0;
                        physics_loop.pause();
                        info!("Pause {} ticks", paused.0);
                    } else {
                        sync_queue.push(adjustment.clone());
                    }
                }
            }
            _ => (),
        }
    }
}
