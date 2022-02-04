use bevy::prelude::{EventReader, Query};

use crate::space_core::{ecs::pawn::{components::StandardCharacter, events::InputToggleCombatMode}};

pub fn toggle_combat_mode (
    mut toggle_combat_mode_events : EventReader<InputToggleCombatMode>,
    mut standard_character_query : Query<&mut StandardCharacter>,
) {
    
    for event in toggle_combat_mode_events.iter() {

        match standard_character_query.get_mut(event.entity) {
            Ok(mut standard_character) => {
                standard_character.combat_mode = !standard_character.combat_mode;
            },
            Err(_rr) => {},
        }

    }

}
