use bevy::prelude::{EventReader, Query};
use networking::messages::InputToggleCombatMode;

use crate::humanoid::Humanoid;

pub fn toggle_combat_mode(
    mut toggle_combat_mode_events: EventReader<InputToggleCombatMode>,
    mut standard_character_query: Query<&mut Humanoid>,
) {
    for event in toggle_combat_mode_events.iter() {
        match standard_character_query.get_mut(event.entity) {
            Ok(mut standard_character) => {
                standard_character.combat_mode = !standard_character.combat_mode;
            }
            Err(_rr) => {}
        }
    }
}
