use std::{collections::HashMap, f32::consts::PI};

use bevy::{math::Vec2, prelude::{Changed, Entity, Query}};

use crate::space_core::{components::{connected_player::ConnectedPlayer, entity_updates::EntityUpdates, inventory::Inventory, inventory_item::InventoryItem, pawn::Pawn, persistent_player_data::PersistentPlayerData, showcase::Showcase, standard_character::{StandardCharacter}}, functions::entity_updates::get_entity_update_difference::get_entity_update_difference, resources::network_messages::EntityUpdateData};

use vector2math::*;

pub fn standard_character_update(
    mut updated_humans: Query<(Entity,Option<&Pawn>, &StandardCharacter, &mut EntityUpdates, &PersistentPlayerData, &Inventory, Option<&ConnectedPlayer>, Option<&Showcase>), Changed<StandardCharacter>>,
    inventory_items : Query<&InventoryItem>,
) {

    for (
        _entity,
        pawn_component_option,
        standard_character_component,
        mut entity_updates_component,
        persistent_player_data_component,
        inventory_component,
        connected_player_component_option,
        showcase_component_option
    ) in updated_humans.iter_mut() {

        let old_entity_updates = entity_updates_component.updates.clone();
        
        let lower_body_animation_state : String;

        let mut upper_body_animation_state : String;
        
        let mut animation_tree1_upper_blend = HashMap::new();
        let mut animation_tree1_lower_body_jogging_strafe_blend_position = HashMap::new();

        let mut upper_body_left_punch_time_scale = HashMap::new();
        let mut upper_body_right_punch_time_scale = HashMap::new();

        upper_body_left_punch_time_scale.insert(
            "time_scale".to_string(),
            EntityUpdateData::Float(2.)
        );
        upper_body_right_punch_time_scale.insert(
            "time_scale".to_string(),
            EntityUpdateData::Float(2.)
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/Punching Left/TimeScale/scale".to_string(),
            upper_body_left_punch_time_scale
        );
        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/Punching Right/TimeScale/scale".to_string(),
            upper_body_right_punch_time_scale
        );

        if standard_character_component.combat_mode {

            // Here we can set upper body animations to certain combat state, eg boxing stance, melee weapon stances, projectile weapon stances etc.

            let mut upper_body_blend_amount = 1.;

            match standard_character_component.current_lower_animation_state {
                crate::space_core::components::standard_character::CharacterAnimationState::Idle => {
                    upper_body_animation_state = "Idle Heightened".to_string();
                    lower_body_animation_state = "Idle".to_string();
                }
                crate::space_core::components::standard_character::CharacterAnimationState::Jogging => {
                    upper_body_animation_state = "JoggingStrafe".to_string();
                    lower_body_animation_state = "JoggingStrafe".to_string();

                    let mut strafe_jogging_blend_position;

                    match pawn_component_option.as_ref().unwrap().facing_direction {
                        crate::space_core::components::pawn::FacingDirection::UpLeft => {
                            strafe_jogging_blend_position = [-1.,1.];
                        },
                        crate::space_core::components::pawn::FacingDirection::Up => {
                            strafe_jogging_blend_position = [0.,1.];
                        },
                        crate::space_core::components::pawn::FacingDirection::UpRight => {
                            strafe_jogging_blend_position = [1.,1.];
                        },
                        crate::space_core::components::pawn::FacingDirection::Right => {
                            strafe_jogging_blend_position = [1.,0.];
                        },
                        crate::space_core::components::pawn::FacingDirection::DownRight => {
                            strafe_jogging_blend_position = [1.,-1.];
                        },
                        crate::space_core::components::pawn::FacingDirection::Down => {
                            strafe_jogging_blend_position = [0.,-1.];
                        },
                        crate::space_core::components::pawn::FacingDirection::DownLeft => {
                            strafe_jogging_blend_position = [-1.,-1.];
                        },
                        crate::space_core::components::pawn::FacingDirection::Left => {
                            strafe_jogging_blend_position = [-1.,0.];
                        },
                    }

                    // Further rotate this Vec2 with mouse_direction.
                    if standard_character_component.facing_direction > 0.75*PI {
                        // Left down
                        strafe_jogging_blend_position = strafe_jogging_blend_position.rotate(-0.75*PI);

                    } else if standard_character_component.facing_direction > 0.5*PI {
                        // Down
                        strafe_jogging_blend_position = strafe_jogging_blend_position.rotate(-1.*PI);

                    } else if standard_character_component.facing_direction > 0.25*PI {
                        // Right down
                        strafe_jogging_blend_position = strafe_jogging_blend_position.rotate(0.75*PI);

                    } else if standard_character_component.facing_direction > 0. {
                        // Right
                        strafe_jogging_blend_position = strafe_jogging_blend_position.rotate(0.5*PI);

                    } else if standard_character_component.facing_direction > -0.25*PI {
                        // Left
                        strafe_jogging_blend_position = strafe_jogging_blend_position.rotate(-0.5*PI);

                    } else if standard_character_component.facing_direction > -0.5*PI {
                        // Left up
                        strafe_jogging_blend_position = strafe_jogging_blend_position.rotate(-0.25*PI);

                    } else if standard_character_component.facing_direction > -0.75*PI {
                        //Up
                        strafe_jogging_blend_position = strafe_jogging_blend_position.rotate(0.);

                    } else if standard_character_component.facing_direction > -1.*PI {
                        //Right Up
                        strafe_jogging_blend_position = strafe_jogging_blend_position.rotate(0.25*PI);

                    }

                    animation_tree1_lower_body_jogging_strafe_blend_position.insert(
                        "blend_position".to_string(),
                        EntityUpdateData::Vec2(Vec2::new(strafe_jogging_blend_position.x(),strafe_jogging_blend_position.y())),
                    );

                }
                crate::space_core::components::standard_character::CharacterAnimationState::Sprinting => {
                    upper_body_animation_state = "Idle Heightened".to_string();
                    lower_body_animation_state = "Sprinting".to_string();
                    upper_body_blend_amount=0.;
                },
            }

            if standard_character_component.is_attacking {

                // Get current active inventory item and animation state
                // Then based on if its left or right handed change upper_body_animation_state.

                let mut combat_item = None;

                let is_left_handed;

                if inventory_component.active_slot == "left_hand" {
                    is_left_handed=true;
                } else {
                    is_left_handed=false;
                }  

                for slot in inventory_component.slots.iter() {

                    if slot.slot_name == inventory_component.active_slot {

                        combat_item = slot.slot_item;
                        break;

                    }

                }


                match combat_item {
                    Some(entity) => {
                        let inventory_item = inventory_items.get(entity).unwrap();

                        match inventory_item.combat_animation {
                            crate::space_core::components::inventory_item::CombatAnimation::OneHandedMeleePunch => {

                                if is_left_handed {
                                    upper_body_animation_state = "Punching Left".to_string();
                                } else {
                                    upper_body_animation_state = "Punching Right".to_string();
                                }

                            },
                        }

                    },
                    None => {
                        if is_left_handed {
                            upper_body_animation_state = "Punching Left".to_string();
                        } else {
                            upper_body_animation_state = "Punching Right".to_string();
                        }
                    },
                }
                
                

            }
            animation_tree1_upper_blend.insert(
                "blend_amount".to_string(),
                EntityUpdateData::Float(upper_body_blend_amount)
            );

        } else {

            match standard_character_component.current_lower_animation_state {
                crate::space_core::components::standard_character::CharacterAnimationState::Idle => {
                    lower_body_animation_state = "Idle".to_string();
                    upper_body_animation_state = "Idle Heightened".to_string();
                }
                crate::space_core::components::standard_character::CharacterAnimationState::Jogging => {
                    lower_body_animation_state = "Jogging".to_string();
                    upper_body_animation_state = "Jogging".to_string();
                }
                crate::space_core::components::standard_character::CharacterAnimationState::Sprinting => {
                    lower_body_animation_state = "Sprinting".to_string();
                    upper_body_animation_state = "Idle Heightened".to_string();
                },
            }

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

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/mainBodyState/JoggingStrafe/BlendSpace2D/blend_position".to_string(),
            animation_tree1_lower_body_jogging_strafe_blend_position
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

        entity_updates_component.updates_difference.push(difference_updates);

    }

    

}
