use bevy::prelude::{warn, Query, Res, ResMut};
use bevy_renet::renet::RenetServer;
use bincode::serialize;

use crate::core::connected_player::{connection::ConnectedPlayer, plugin::HandleToEntity};

use super::{
    networking::ReliableServerMessage,
    plugin::{NetEvent, RENET_RELIABLE_CHANNEL_ID},
};

pub fn send_net(
    net: &mut ResMut<RenetServer>,
    connected_players: &Query<&ConnectedPlayer>,
    handle_to_entity: &Res<HandleToEntity>,
    new_event: &NetEvent,
) {
    let mut connected = false;

    match handle_to_entity.map.get(&new_event.handle) {
        Some(r) => match connected_players.get(*r) {
            Ok(rr) => {
                if rr.connected {
                    connected = true;
                }
            }
            Err(_rr) => {
                warn!(
                    "Couldnt get handle from HandleToEntity for {:?} , message: {:?}",
                    r, new_event.message
                );
                return;
            }
        },
        None => {
            warn!("Couldnt find handle entity!");
            return;
        }
    }
    if !connected {
        return;
    }

    net.send_message(
        new_event.handle,
        RENET_RELIABLE_CHANNEL_ID,
        serialize::<ReliableServerMessage>(&new_event.message).unwrap(),
    );
}
