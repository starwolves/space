use crate::controller::ControllerInput;
use bevy::prelude::{EventReader, Res};
use player::connection::SendServerConfiguration;

#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands: Commands,
) {
    for event in config_events.iter() {
        match handle_to_entity.map.get(&event.handle) {
            Some(entity) => {
                commands.entity(*entity).insert(ControllerInput::default());
            }
            None => {}
        }
    }
}

use bevy::prelude::warn;

use bevy::prelude::Vec2;
use bevy::prelude::{Commands, Query, ResMut};
use bevy_renet::renet::ServerEvent;
use combat::health_ui::ClientHealthUICache;
use networking::server::{ConnectedPlayer, HandleToEntity};
use player::boarding::PersistentPlayerData;
use player::names::UsedNames;

/// Manage client connection events.
#[cfg(feature = "server")]
pub(crate) fn connections(
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut reader: EventReader<ServerEvent>,
    mut used_names: ResMut<UsedNames>,
    mut connected_players: Query<(
        &mut PersistentPlayerData,
        &mut ConnectedPlayer,
        &mut ControllerInput,
    )>,
    mut client_health_ui_cache: ResMut<ClientHealthUICache>,
) {
    for event in reader.iter() {
        match event {
            ServerEvent::ClientConnected(_, _) => {}
            ServerEvent::ClientDisconnected(handle) => {
                let entity;
                match handle_to_entity.map.get(handle) {
                    Some(ent) => {
                        entity = Some(*ent);
                        match connected_players.get_mut(*ent) {
                            Ok((
                                mut persistent_player_data,
                                mut connected_player_component,
                                mut player_input_component,
                            )) => {
                                connected_player_component.connected = false;
                                player_input_component.movement_vector = Vec2::ZERO;
                                player_input_component.sprinting = false;
                                player_input_component.is_mouse_action_pressed = false;
                                player_input_component.auto_move_enabled = false;

                                // When reconnecting into the old pawn works remove this.
                                used_names
                                    .account_name
                                    .remove(&persistent_player_data.account_name);
                                persistent_player_data.account_name =
                                    "disconnectedUser".to_string();
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
                handle_to_entity.map.remove(handle);
                match entity {
                    Some(ent) => {
                        handle_to_entity.inv_map.remove(&ent);
                        client_health_ui_cache.cache.remove(&ent);
                    }
                    None => {}
                }
            }
        }
    }
}
