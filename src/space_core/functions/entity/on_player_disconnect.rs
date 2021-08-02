use bevy::prelude::{Query, ResMut};

use crate::space_core::{components::connected_player::ConnectedPlayer, resources::handle_to_entity::HandleToEntity};

pub fn on_player_disconnect(
    handle : u32,
    handle_to_entity : &mut ResMut<HandleToEntity>,
    connected_players : &mut Query<&mut ConnectedPlayer>,
) {

    let mut entity = None;

    match handle_to_entity.map.get(&handle) {
        Some(ent) => {
            entity = Some(*ent);
            match connected_players.get_mut(*ent) {
                Ok(mut connected_player_component) => {

                    connected_player_component.connected = false;

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
