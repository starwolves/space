use bevy_app::EventReader;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_networking_turbulence::NetworkResource;

use crate::core::{
    atmospherics::events::{
        NetAtmosphericsNotices, NetMapDisplayAtmospherics, NetMapHoverAtmospherics,
    },
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    networking::{send_net, NetEvent},
};

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
