use bevy::{math::Vec2, prelude::{Query, ResMut, info}};

use crate::space_core::{components::{connected_player::ConnectedPlayer, persistent_player_data::PersistentPlayerData, player_input::PlayerInput}, resources::{client_health_ui_cache::ClientHealthUICache, handle_to_entity::HandleToEntity, used_names::UsedNames}};

pub fn on_player_disconnect(
    handle : u32,
    handle_to_entity : &mut ResMut<HandleToEntity>,
    connected_players : &mut Query<(&mut PersistentPlayerData, &mut ConnectedPlayer, &mut PlayerInput)>,
    used_names : &mut ResMut<UsedNames>,
    client_health_ui_cache : &mut ResMut<ClientHealthUICache>,
) {

    info!("[{}] disconnected!", handle);

    let mut entity = None;

    match handle_to_entity.map.get(&handle) {
        Some(ent) => {
            entity = Some(*ent);
            match connected_players.get_mut(*ent) {
                Ok((mut persistent_player_data, mut connected_player_component, mut player_input_component)) => {

                    connected_player_component.connected = false;
                    player_input_component.movement_vector = Vec2::ZERO;
                    player_input_component.sprinting = false;
                    player_input_component.is_mouse_action_pressed = false;
                    player_input_component.auto_move_enabled = false;

                    // When reconnecting into the old pawn works remove this.
                    used_names.user_names.remove(&persistent_player_data.user_name);
                    persistent_player_data.user_name = "disconnectedUser".to_string();

                },
                Err(_rr) => {},
            }
            
        },
        None => {},
    }

    match entity {
        Some(ent) => {
            handle_to_entity.inv_map.remove(&ent);
            client_health_ui_cache.cache.remove(&ent);
        },
        None => {},
    }

    handle_to_entity.map.remove(&handle);

}
