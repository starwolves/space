use bevy_app::EventReader;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_networking_turbulence::NetworkResource;

use crate::{
    core::{
        connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
        networking::{send_net, NetEvent},
    },
    entities::counter_windows::events::NetCounterWindow,
};

pub fn net_system(
    mut net: ResMut<NetworkResource>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetCounterWindow>,
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
}
