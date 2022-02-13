use bevy::prelude::{Commands, EventReader, Query, Res};

use crate::space::{core::{pawn::{components::{Boarding, SoftPlayer}, events::InputUIInput, resources::HandleToEntity}, networking::resources::{UIInputNodeClass, UIInputAction}}};

pub fn ui_input_event(
    mut event : EventReader<InputUIInput>,
    handle_to_entity: Res<HandleToEntity>,
    criteria_query : Query<&SoftPlayer>,
    mut commands : Commands
) {

    

    for new_event in event.iter() {

        let player_entity = handle_to_entity.map.get(&new_event.handle)
        .expect("ui_input_event.rs could not find components for player that just got done boarding.");

        // Safety check.
        match criteria_query.get(*player_entity) {
            Ok(_) => {},
            Err(_rr) => {continue;},
        }

        if new_event.ui_type == "setupUI" {

            if new_event.node_name == "board" && 
            matches!(new_event.node_class, UIInputNodeClass::Button) && 
            matches!(new_event.action, UIInputAction::Pressed) {

                commands.entity(*player_entity).insert(Boarding);

            }

        }

    }

}
