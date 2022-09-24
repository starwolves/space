use api::{
    data::{ConnectedPlayer, HandleToEntity},
    network::{PendingNetworkMessage, ReliableServerMessage},
};
use bevy::prelude::{warn, EventReader, Query, Res, ResMut};
use bevy_renet::renet::RenetServer;
use bincode::serialize;
use networking::plugin::{NetEvent, RENET_RELIABLE_CHANNEL_ID};

/// Manage sending netcode messages.
pub(crate) fn send_net(
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
                connected = true;
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

/// Process netcode messages.
pub(crate) fn process_net(
    mut pending_network_message: EventReader<PendingNetworkMessage>,
    connected_players: Query<&ConnectedPlayer>,
    mut net: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for p in pending_network_message.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: p.handle,
                message: p.message.clone(),
            },
        );
    }
}
