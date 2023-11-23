use crate::controller::ControllerInput;

use bevy::log::warn;
use bevy::prelude::Res;

use bevy::prelude::Vec2;
use bevy::prelude::{Query, ResMut};
use bevy_renet::renet::ServerEvent;
use networking::server::{ConnectedPlayer, HandleToEntity};
use player::connections::ServerEventBuffer;

/// Manage client connection events.
#[allow(unused_variables)]
pub(crate) fn connections(
    mut handle_to_entity: ResMut<HandleToEntity>,
    buffer: Res<ServerEventBuffer>,
    mut connected_players: Query<(&mut ConnectedPlayer, &mut ControllerInput)>,
) {
    for e in buffer.buffer.iter() {
        let event = e.renet_event();
        match event {
            ServerEvent::ClientConnected { client_id } => {}
            ServerEvent::ClientDisconnected { client_id, reason } => {
                let entity;
                match handle_to_entity.map.get(&client_id) {
                    Some(ent) => {
                        entity = Some(*ent);
                        match connected_players.get_mut(*ent) {
                            Ok((mut connected_player_component, mut player_input_component)) => {
                                connected_player_component.connected = false;
                                player_input_component.movement_vector = Vec2::ZERO;
                            }
                            Err(_) => {
                                warn!("Couldnt find proper components of player entity.");
                            }
                        }
                    }
                    None => {
                        warn!("Couldnt find entity from handle");
                        continue;
                    }
                }
                handle_to_entity.map.remove(&client_id);
                match entity {
                    Some(ent) => {
                        handle_to_entity.inv_map.remove(&ent);
                    }
                    None => {}
                }
            }
        }
    }
}
