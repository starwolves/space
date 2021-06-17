use std::collections::HashMap;

use bevy::prelude::{Changed, Query};

use crate::space_core::{components::{connected_player::ConnectedPlayer, entity_updates::EntityUpdates, human_character::{HumanCharacter}, persistent_player_data::PersistentPlayerData}, structs::network_messages::EntityUpdateData};

pub fn human_pawn_update(
    mut updated_humans: Query<(&HumanCharacter, &mut EntityUpdates, &PersistentPlayerData, Option<&ConnectedPlayer>), Changed<HumanCharacter>>,
) {

    for (
        human_character_component,
        mut entity_updates_component,
        persistent_player_data_component,
        connected_player_component_option
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

        let mut billboard_username_updates = HashMap::new();

        billboard_username_updates.insert(
            "bbcode".to_string(),
            EntityUpdateData::String("[color=white][center][b]".to_owned() + &persistent_player_data_component.character_name + "[/b][/center][/color]")
        );

        match connected_player_component_option {
            Some(connected_player_component) => {
                entity_updates_component.excluded_handles.insert("Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText/VControl/name".to_string(), vec![connected_player_component.handle]);
            },
            None => {},
        }

        

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText/VControl/name".to_string(),
            billboard_username_updates
        );

    }

    

}
