use bevy::prelude::{EventReader, Query};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::{components::standard_character::StandardCharacter, events::general::mouse_direction_update::MouseDirectionUpdate};

pub fn mouse_direction_update(
    mut update_events : EventReader<MouseDirectionUpdate>,
    standard_characters : Query<(&RigidBodyPosition, &StandardCharacter)>,
) {

    for event in update_events.iter() {

        match standard_characters.get(event.entity) {
            Ok((rigid_body_position_component, standard_character_component)) => {

                if standard_character_component.combat_mode == false{
                    continue;
                }


                


            },
            Err(_rr) => {},
        }

    }

}
