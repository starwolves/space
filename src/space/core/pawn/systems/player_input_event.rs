use bevy_internal::prelude::{warn, EventReader, Query, Res};

use crate::space::core::pawn::{
    components::PlayerInput,
    events::{InputMovementInput, InputSprinting},
    resources::HandleToEntity,
};

pub fn player_input_event(
    mut movement_input_event: EventReader<InputMovementInput>,
    mut sprinting_input_event: EventReader<InputSprinting>,
    handle_to_entity: Res<HandleToEntity>,
    mut query: Query<&mut PlayerInput>,
) {
    for new_event in movement_input_event.iter() {
        let player_entity = handle_to_entity
            .map
            .get(&new_event.handle)
            .expect("movement_input_event.rs could not find player entity belonging to handle.");

        let player_input_component_result = query.get_mut(*player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                player_input_component.movement_vector = new_event.vector;
            }
            Err(_rr) => {
                warn!("Couldn't process player input (movement_input_event): couldn't find player_entity.");
            }
        }
    }

    for new_event in sprinting_input_event.iter() {
        let player_entity = handle_to_entity
            .map
            .get(&new_event.handle)
            .expect("movement_input_event.rs could not find player entity belonging to handle.");

        let player_input_component_result = query.get_mut(*player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                player_input_component.sprinting = new_event.is_sprinting;
            }
            Err(_rr) => {
                warn!("Couldn't process player input (sprinting_input_event): couldn't find player_entity.");
            }
        }
    }
}
