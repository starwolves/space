use bevy_app::EventReader;
use bevy_ecs::system::{Query, Res, ResMut};
use bevy_networking_turbulence::NetworkResource;

use crate::space::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    networking::{
        resources::{GridMapType, NetProjectileType, ReliableServerMessage},
        send_net, NetEvent,
    },
};

use super::resources::{CellData, Vec3Int};

pub struct RemoveCell {
    pub handle_option: Option<u32>,
    pub gridmap_type: GridMapType,
    pub id: Vec3Int,
    pub cell_data: CellData,
}

pub struct NetGridmapUpdates {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct ProjectileFOV {
    pub laser_projectile: NetProjectileType,
}

pub struct NetProjectileFOV {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub fn net_system(
    mut net: ResMut<NetworkResource>,
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
