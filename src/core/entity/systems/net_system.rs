use bevy_ecs::{
    event::EventReader,
    system::{Query, Res, ResMut},
};
use bevy_renet::renet::RenetServer;

use crate::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    entity::events::{NetLoadEntity, NetSendEntityUpdates, NetShowcase, NetUnloadEntity},
    networking::{send_net, NetEvent},
};

pub fn net_system(
    mut net: ResMut<RenetServer>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetLoadEntity>,
    mut net2: EventReader<NetUnloadEntity>,
    mut net3: EventReader<NetSendEntityUpdates>,
    mut net4: EventReader<NetShowcase>,
) {
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
    for new_event in net3.iter() {
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
    for new_event in net4.iter() {
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
