use bevy_app::EventReader;
use bevy_ecs::{
    entity::Entity,
    system::{Query, Res, ResMut},
};
use bevy_math::Vec2;
use bevy_networking_turbulence::NetworkResource;

use crate::space::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    networking::{resources::ReliableServerMessage, send_net, NetEvent},
};

pub struct InputMapChangeDisplayMode {
    pub handle: u32,
    pub entity: Entity,
    pub display_mode: String,
}

pub struct InputMapRequestDisplayModes {
    pub handle: u32,
    pub entity: Entity,
}

pub struct NetRequestDisplayModes {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct InputMap {
    pub handle: u32,
    pub entity: Entity,
    pub input: MapInput,
}

pub enum MapInput {
    Range(f32),
    Position(Vec2),
    MouseCell(i16, i16),
}

pub fn net_system(
    mut net: ResMut<NetworkResource>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetRequestDisplayModes>,
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
