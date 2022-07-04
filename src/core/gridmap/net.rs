use bevy::prelude::{EventReader, Query, Res, ResMut};
use bevy_renet::renet::RenetServer;

use crate::core::{
    connected_player::{connection::ConnectedPlayer, plugin::HandleToEntity},
    networking::{net::send_net, networking::ReliableServerMessage, plugin::NetEvent},
};

pub struct NetGridmapUpdates {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetProjectileFOV {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub fn net_system(
    mut net: ResMut<RenetServer>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetProjectileFOV>,
    mut net2: EventReader<NetGridmapUpdates>,
) {
    for new_event in net1.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net2.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
}
