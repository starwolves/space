use std::collections::HashMap;

use bevy::prelude::{Changed, Query};

use crate::space_core::{components::{entity_updates::EntityUpdates, human_character::{HumanCharacter}}, structs::network_messages::EntityUpdateData};

pub fn human_pawn_update(
    mut updated_humans: Query<(&HumanCharacter, &mut EntityUpdates), Changed<HumanCharacter>>,
) {

    for (
        human_character_component,
        mut entity_updates_component
    ) in updated_humans.iter_mut() {

        
        let lower_body_animation_state : String;
        let upper_body_animation_state : String;

        

        match human_character_component.state {
            crate::space_core::components::human_character::State::Idle => {
                lower_body_animation_state = "Idle".to_string();
                upper_body_animation_state = "Idle".to_string();
            }
            crate::space_core::components::human_character::State::Walking => {
                lower_body_animation_state = "Jogging".to_string();
                upper_body_animation_state = "Jogging".to_string();
            }
        }

        // Check how this works on client intepretation.
        // Junen voor baan eerst.

        let mut animation_tree1_upper_body_updates = HashMap::new();
        let mut animation_tree1_lower_body_updates = HashMap::new();

        animation_tree1_upper_body_updates.insert(
            "travel".to_string(),
            EntityUpdateData::String(upper_body_animation_state)
        );
        animation_tree1_lower_body_updates.insert(
            "travel".to_string(),
            EntityUpdateData::String(lower_body_animation_state)
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/playback/travel".to_string(),
            animation_tree1_upper_body_updates
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/mainBodyState/playback/travel".to_string(),
            animation_tree1_lower_body_updates
        );


    }

    

}
