use bevy_app::EventReader;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_networking_turbulence::NetworkResource;

use crate::space::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    networking::{resources::ReliableServerMessage, send_net, NetEvent},
};

pub struct NetMapDisplayAtmospherics {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetMapHoverAtmospherics {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetAtmosphericsNotices {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub fn net_system(
    mut net: ResMut<NetworkResource>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetMapDisplayAtmospherics>,
    mut net2: EventReader<NetMapHoverAtmospherics>,
    mut net3: EventReader<NetAtmosphericsNotices>,
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
}
