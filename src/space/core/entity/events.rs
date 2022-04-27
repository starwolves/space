use bevy_app::EventReader;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_networking_turbulence::NetworkResource;

use crate::space::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    networking::{resources::ReliableServerMessage, send_net, NetEvent},
};

pub struct NetLoadEntity {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetShowcase {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetUnloadEntity {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetSendEntityUpdates {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub fn net_system(
    mut net: ResMut<NetworkResource>,
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
