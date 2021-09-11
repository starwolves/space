use bevy::{math::Vec2, prelude::{Query, ResMut}};

use crate::space_core::{components::{connected_player::ConnectedPlayer, player_input::PlayerInput}, resources::handle_to_entity::HandleToEntity};

pub fn on_player_disconnect(
    handle : u32,
    handle_to_entity : &mut ResMut<HandleToEntity>,
    connected_players : &mut Query<(&mut ConnectedPlayer, &mut PlayerInput)>,
) {

    let mut entity = None;

    match handle_to_entity.map.get(&handle) {
        Some(ent) => {
            entity = Some(*ent);
            match connected_players.get_mut(*ent) {
                Ok((mut connected_player_component, mut player_input_component)) => {

                    connected_player_component.connected = false;
                    player_input_component.movement_vector = Vec2::ZERO;
                    player_input_component.sprinting = false;
                    player_input_component.is_mouse_action_pressed = false;
                    player_input_component.auto_move_enabled = false;

                },
                Err(_rr) => {},
            }
            
        },
        None => {},
    }

    match entity {
        Some(ent) => {
            handle_to_entity.inv_map.remove(&ent);
        },
        None => {},
    }

    handle_to_entity.map.remove(&handle);

}
