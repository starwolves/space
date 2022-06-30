use bevy_ecs::{
    event::EventReader,
    system::{Query, Res, ResMut},
};
use bevy_renet::renet::RenetServer;

use crate::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    inventory::events::{
        NetDropCurrentItem, NetPickupWorldItem, NetSwitchHands, NetTakeOffItem, NetThrowItem,
        NetWearItem,
    },
    networking::{send_net, NetEvent, RENET_RELIABLE_CHANNEL_ID},
};

pub fn net_system(
    mut net: ResMut<RenetServer>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetPickupWorldItem>,
    mut net2: EventReader<NetDropCurrentItem>,
    mut net3: EventReader<NetSwitchHands>,
    mut net4: EventReader<NetWearItem>,
    mut net5: EventReader<NetTakeOffItem>,
    mut net6: EventReader<NetThrowItem>,
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
            RENET_RELIABLE_CHANNEL_ID,
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
            RENET_RELIABLE_CHANNEL_ID,
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
            RENET_RELIABLE_CHANNEL_ID,
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
            RENET_RELIABLE_CHANNEL_ID,
        );
    }
    for new_event in net5.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
            RENET_RELIABLE_CHANNEL_ID,
        );
    }
    for new_event in net6.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
            RENET_RELIABLE_CHANNEL_ID,
        );
    }
}
