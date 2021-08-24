use std::collections::HashMap;

use bevy::prelude::{Changed, Entity, Query};

use crate::space_core::{components::{connected_player::ConnectedPlayer, entity_updates::EntityUpdates, persistent_player_data::PersistentPlayerData, showcase::Showcase, standard_character::{StandardCharacter}}, functions::entity_updates::get_entity_update_difference::get_entity_update_difference, resources::network_messages::EntityUpdateData};

pub fn standard_character_update(
    mut updated_humans: Query<(Entity,&StandardCharacter, &mut EntityUpdates, &PersistentPlayerData, Option<&ConnectedPlayer>, Option<&Showcase>), Changed<StandardCharacter>>,
) {

    for (
        _entity,
        standard_character_component,
        mut entity_updates_component,
        persistent_player_data_component,
        connected_player_component_option,
        showcase_component_option
    ) in updated_humans.iter_mut() {

        let old_entity_updates = entity_updates_component.updates.clone();
        
        let lower_body_animation_state : String;

        let mut upper_body_animation_state : String;

        match standard_character_component.current_animation_state {
            crate::space_core::components::standard_character::CharacterAnimationState::Idle => {
                lower_body_animation_state = "Idle".to_string();
                upper_body_animation_state = "Idle".to_string();
            }
            crate::space_core::components::standard_character::CharacterAnimationState::Walking => {
                lower_body_animation_state = "Jogging".to_string();
                upper_body_animation_state = "Idle".to_string();
            }
            crate::space_core::components::standard_character::CharacterAnimationState::Sprinting => {
                lower_body_animation_state = "Sprinting".to_string();
                upper_body_animation_state = "Idle".to_string();
            },
        }

        
        let mut animation_tree1_upper_blend = HashMap::new();

        if standard_character_component.combat_mode {

            // Here we can set upper body animations to certain combat state, eg boxing stance, melee weapon stances, projectile weapon stances etc.

            match standard_character_component.current_animation_state {
                crate::space_core::components::standard_character::CharacterAnimationState::Idle => {
                    upper_body_animation_state = "Idle".to_string();
                }
                crate::space_core::components::standard_character::CharacterAnimationState::Walking => {
                    upper_body_animation_state = "Idle".to_string();
                }
                crate::space_core::components::standard_character::CharacterAnimationState::Sprinting => {
                    upper_body_animation_state = "Idle".to_string();
                },
            }

            
            // 0 for now.
            animation_tree1_upper_blend.insert(
                "blend_amount".to_string(),
                EntityUpdateData::Float(0.)
            );

        } else {

            animation_tree1_upper_blend.insert(
                "blend_amount".to_string(),
                EntityUpdateData::Float(0.)
            );

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

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyBlend/blend_amount".to_string(),
            animation_tree1_upper_blend
        );
        

        match showcase_component_option {
            Some(_showcase_component) => {
            },
            None => {
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

            },
        }

        

        let difference_updates = get_entity_update_difference(
            old_entity_updates,
            &entity_updates_component.updates
        );

        entity_updates_component.updates_difference = difference_updates;

    }

    

}
