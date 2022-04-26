use bevy_ecs::system::{Query, ResMut};
use bevy_log::info;
use bevy_math::Vec2;

use crate::space::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    health::resources::ClientHealthUICache,
    humanoid::components::{CharacterAnimationState, Humanoid},
    pawn::{
        components::{ControllerInput, PersistentPlayerData},
        resources::UsedNames,
    },
};

pub fn on_player_disconnect(
    handle: u32,
    handle_to_entity: &mut ResMut<HandleToEntity>,
    connected_players: &mut Query<(
        &mut PersistentPlayerData,
        &mut ConnectedPlayer,
        &mut ControllerInput,
        &mut Humanoid,
    )>,
    used_names: &mut ResMut<UsedNames>,
    client_health_ui_cache: &mut ResMut<ClientHealthUICache>,
) {
    info!("[{}] disconnected!", handle);

    let mut entity = None;

    match handle_to_entity.map.get(&handle) {
        Some(ent) => {
            entity = Some(*ent);
            match connected_players.get_mut(*ent) {
                Ok((
                    mut persistent_player_data,
                    mut connected_player_component,
                    mut player_input_component,
                    mut standard_character_component,
                )) => {
                    standard_character_component.current_lower_animation_state =
                        CharacterAnimationState::Idle;
                    connected_player_component.connected = false;
                    player_input_component.movement_vector = Vec2::ZERO;
                    player_input_component.sprinting = false;
                    player_input_component.is_mouse_action_pressed = false;
                    player_input_component.auto_move_enabled = false;

                    // When reconnecting into the old pawn works remove this.
                    used_names
                        .user_names
                        .remove(&persistent_player_data.user_name);
                    persistent_player_data.user_name = "disconnectedUser".to_string();
                }
                Err(_rr) => {}
            }
        }
        None => {}
    }

    match entity {
        Some(ent) => {
            handle_to_entity.inv_map.remove(&ent);
            client_health_ui_cache.cache.remove(&ent);
        }
        None => {}
    }

    handle_to_entity.map.remove(&handle);
}
