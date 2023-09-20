use bevy::prelude::{info, EventReader, Local, ResMut};
use bevy_xpbd_3d::prelude::PhysicsLoop;
use networking::{client::IncomingReliableServerMessage, server::NetworkingServerMessage};

pub(crate) fn pause_loop(
    mut net: EventReader<IncomingReliableServerMessage<NetworkingServerMessage>>,
    mut physics_loop: ResMut<PhysicsLoop>,
    mut paused: Local<(i8, i8)>,
) {
    if physics_loop.paused {
        paused.1 += 1;
        if paused.1 >= paused.0 {
            physics_loop.resume();
        }
    }

    for message in net.iter() {
        match &message.message {
            NetworkingServerMessage::AdjustSync(adjustment) => {
                if adjustment.advance > 0 {
                    paused.0 = adjustment.advance;
                    paused.1 = 0;
                    physics_loop.pause();
                    info!("Pause {} ticks", paused.0);
                }
            }
            _ => (),
        }
    }
}
