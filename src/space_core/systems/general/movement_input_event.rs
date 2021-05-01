use bevy::prelude::{EventReader, Query, Res};

use crate::space_core::{components::player_input::PlayerInput, events::general::movement_input::MovementInput, resources::handle_to_entity::HandleToEntity};

pub fn movement_input_event(
    mut event : EventReader<MovementInput>,
    handle_to_entity: Res<HandleToEntity>,
    mut query : Query<&mut PlayerInput>
) {

    for new_event in event.iter() {

        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("movement_input_event.rs could not find player entity belonging to handle.");

        let mut player_input_component = query.get_mut(*player_entity)
        .expect("movement_input_event.rs could not find player entity in query.");

        player_input_component.movement_vector = new_event.vector;


    }

}
