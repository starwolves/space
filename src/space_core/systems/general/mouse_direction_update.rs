use bevy::prelude::{EventReader, Query};

use crate::space_core::{components::standard_character::StandardCharacter, events::general::mouse_direction_update::MouseDirectionUpdate};

pub fn mouse_direction_update(
    mut update_events : EventReader<MouseDirectionUpdate>,
    mut standard_characters : Query<&mut StandardCharacter>,
) {

    for event in update_events.iter() {

        match standard_characters.get_mut(event.entity) {
            Ok(mut standard_character_component) => {

                if standard_character_component.combat_mode == false{
                    continue;
                }

                standard_character_component.facing_direction = event.direction;

            },
            Err(_rr) => {},
        }

    }

}
