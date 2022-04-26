use bevy_ecs::{
    entity::Entity,
    system::{Query, ResMut},
};
use bevy_math::Vec3;
use bevy_networking_turbulence::NetworkResource;

use crate::space::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    networking::{resources::ReliableServerMessage, send_net, NetEvent},
};

pub struct InputDropCurrentItem {
    pub pickuper_entity: Entity,
    pub input_position_option: Option<Vec3>,
}

pub struct InputThrowItem {
    pub entity: Entity,
    pub position: Vec3,
    pub angle: f32,
}

pub struct InputSwitchHands {
    pub entity: Entity,
}

pub struct InputTakeOffItem {
    pub entity: Entity,
    pub slot_name: String,
}

pub struct InputUseWorldItem {
    pub pickuper_entity: Entity,
    pub pickupable_entity_bits: u64,
}

pub struct InputWearItem {
    pub wearer_entity: Entity,
    pub wearable_id_bits: u64,
    pub wear_slot: String,
}

pub struct NetDropCurrentItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetPickupWorldItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetSwitchHands {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetTakeOffItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetThrowItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetWearItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

use bevy_app::{EventReader, EventWriter};
use bevy_ecs::system::Res;

use crate::space::core::tab_actions::resources::QueuedTabActions;

pub fn inventory_actions(
    queue: Res<QueuedTabActions>,

    mut pickup_world_item_event: EventWriter<InputUseWorldItem>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "actions::inventory/pickup" {
            if queued.target_entity_option.is_some() {
                pickup_world_item_event.send(InputUseWorldItem {
                    pickuper_entity: queued.player_entity,
                    pickupable_entity_bits: queued.target_entity_option.unwrap(),
                });
            }
        }
    }
}

pub fn net_system(
    mut net: ResMut<NetworkResource>,
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
    for new_event in net5.iter() {
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
    for new_event in net6.iter() {
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
