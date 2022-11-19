use std::{collections::HashMap, f32::consts::PI};

use bevy::{
    math::Vec2,
    prelude::{Changed, Entity, Query},
};
use entity::entity_data::{get_entity_update_difference, EntityUpdates};
use inventory_api::core::Inventory;
use networking::server::EntityUpdateData;
use pawn::pawn::FacingDirection;
use showcase::core::Showcase;

use inventory_item::{
    combat::{CombatAttackAnimation, MeleeCombat, ProjectileCombat},
    item::CombatStandardAnimation,
};

use inventory_item::item::InventoryItem;
use pawn::pawn::Pawn;
use vector2math::{FloatingVector2, Vector2};

use crate::humanoid::{CharacterAnimationState, Humanoid};
use controller::controller::ControllerInput;
use networking::server::ConnectedPlayer;
use player::boarding::PersistentPlayerData;

/// All the core humanoid entity updates for the Godot client.
#[cfg(feature = "server")]
pub(crate) fn humanoid_core_entity_updates(
    mut updated_humans: Query<
        (
            Entity,
            Option<&Pawn>,
            &Humanoid,
            &mut EntityUpdates,
            &PersistentPlayerData,
            &Inventory,
            Option<&ControllerInput>,
            Option<&ConnectedPlayer>,
            Option<&Showcase>,
        ),
        Changed<Humanoid>,
    >,
    inventory_items: Query<(&InventoryItem, &MeleeCombat, Option<&ProjectileCombat>)>,
) {
    for (
        _entity,
        pawn_component_option,
        humanoid_component,
        mut entity_updates_component,
        persistent_player_data_component,
        inventory_component,
        player_input_option,
        connected_player_component_option,
        showcase_component_option,
    ) in updated_humans.iter_mut()
    {
        let old_entity_updates = entity_updates_component.updates.clone();

        let lower_body_animation_state: String;

        let mut upper_body_animation_state: String;

        let mut animation_tree1_upper_blend = HashMap::new();
        let mut animation_tree1_lower_body_strafe_blend_position = HashMap::new();
        let mut animation_tree1_upper_body_strafe_blend_position = HashMap::new();

        let mut upper_body_left_punch_time_scale = HashMap::new();
        let mut upper_body_right_punch_time_scale = HashMap::new();

        upper_body_left_punch_time_scale
            .insert("time_scale".to_string(), EntityUpdateData::Float(2.));
        upper_body_right_punch_time_scale
            .insert("time_scale".to_string(), EntityUpdateData::Float(2.));

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/Punching Left/TimeScale/scale".to_string(),
            upper_body_left_punch_time_scale
        );
        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/Punching Right/TimeScale/scale".to_string(),
            upper_body_right_punch_time_scale
        );

        let mut active_item_entity = None;

        let is_left_handed;

        if inventory_component.active_slot == "left_hand" {
            is_left_handed = true;
        } else {
            is_left_handed = false;
        }

        let mut inventory_item_components_option = None;

        for slot in inventory_component.slots.iter() {
            if slot.slot_name == inventory_component.active_slot {
                active_item_entity = slot.slot_item;
                match active_item_entity {
                    Some(ent) => {
                        inventory_item_components_option = Some(inventory_items.get(ent).unwrap());
                    }
                    None => {}
                }

                break;
            }
        }

        let mut lower_body_strafe_blendspace2d_path = "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/JoggingStrafe/BlendSpace2D/blend_position".to_string();
        let upper_body_strafe_blendspace2d_path = "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/mainBodyState/JoggingStrafe/BlendSpace2D/blend_position".to_string();

        let mut update_upper_body = true;

        let mut alt_attack_mode = false;

        if humanoid_component.combat_mode {
            // Here we can set upper body animations to certain combat state, eg boxing stance, melee weapon stances, projectile weapon stances etc.

            let mut upper_body_blend_amount = 1.;

            let projectile_combat_component_option;

            match humanoid_component.current_lower_animation_state {
                CharacterAnimationState::Idle => match inventory_item_components_option {
                    Some((
                        inventory_item_component,
                        _melee_combat_component,
                        projectile_combat_option,
                    )) => {
                        projectile_combat_component_option = projectile_combat_option;

                        match player_input_option {
                            Some(player_input_component) => {
                                if player_input_component.alt_attack_mode
                                    && projectile_combat_component_option.is_some()
                                {
                                    alt_attack_mode = true;
                                }
                            }
                            None => {}
                        }

                        match inventory_item_component.combat_standard_animation {
                            CombatStandardAnimation::StandardStance => {
                                upper_body_animation_state = "Idle Heightened".to_string();
                                lower_body_animation_state = "Idle".to_string();
                            }
                            CombatStandardAnimation::PistolStance => {
                                if !alt_attack_mode {
                                    upper_body_animation_state = "Pistol Idle".to_string();
                                    lower_body_animation_state = "Pistol Idle".to_string();
                                } else {
                                    upper_body_animation_state = "Idle Heightened".to_string();
                                    lower_body_animation_state = "Idle".to_string();
                                }
                            }
                        }
                    }
                    None => {
                        upper_body_animation_state = "Idle Heightened".to_string();
                        lower_body_animation_state = "Idle".to_string();
                    }
                },
                CharacterAnimationState::Jogging => {
                    // Get active item in hand and check its AnimationEnum type. If StandardMelee its JoggingStrafe, if Pistol it's PistolStrafe.

                    match inventory_item_components_option {
                        Some((
                            inventory_item_component,
                            _melee_combat_component,
                            projectile_combat_component_option,
                        )) => {
                            match player_input_option {
                                Some(player_input_component) => {
                                    if player_input_component.alt_attack_mode
                                        && projectile_combat_component_option.is_some()
                                    {
                                        alt_attack_mode = true;
                                    }
                                }
                                None => {}
                            }

                            match inventory_item_component.combat_standard_animation {
                                CombatStandardAnimation::StandardStance => {
                                    upper_body_animation_state = "JoggingStrafe".to_string();
                                    lower_body_animation_state = "JoggingStrafe".to_string();
                                }
                                CombatStandardAnimation::PistolStance => {
                                    if !alt_attack_mode {
                                        upper_body_animation_state = "PistolStrafe".to_string();
                                        lower_body_animation_state = "PistolStrafe".to_string();
                                        //upper_body_strafe_blendspace2d_path = "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/PistolStrafe/BlendSpace2D/blend_position".to_string();
                                        lower_body_strafe_blendspace2d_path = "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/mainBodyState/PistolStrafe/BlendSpace2D/blend_position".to_string();
                                        update_upper_body = false;
                                    } else {
                                        upper_body_animation_state = "JoggingStrafe".to_string();
                                        lower_body_animation_state = "JoggingStrafe".to_string();
                                    }
                                }
                            }
                        }
                        None => {
                            upper_body_animation_state = "JoggingStrafe".to_string();
                            lower_body_animation_state = "JoggingStrafe".to_string();
                        }
                    }

                    let mut strafe_blend_position;

                    match pawn_component_option.as_ref().unwrap().facing_direction {
                        FacingDirection::UpLeft => {
                            strafe_blend_position = [-1., 1.];
                        }
                        FacingDirection::Up => {
                            strafe_blend_position = [0., 1.];
                        }
                        FacingDirection::UpRight => {
                            strafe_blend_position = [1., 1.];
                        }
                        FacingDirection::Right => {
                            strafe_blend_position = [1., 0.];
                        }
                        FacingDirection::DownRight => {
                            strafe_blend_position = [-1., -1.];
                        }
                        FacingDirection::Down => {
                            strafe_blend_position = [0., -1.];
                        }
                        FacingDirection::DownLeft => {
                            strafe_blend_position = [1., -1.];
                        }
                        FacingDirection::Left => {
                            strafe_blend_position = [-1., 0.];
                        }
                    }

                    // Further rotate this Vec2 with mouse_direction.
                    if humanoid_component.facing_direction > 0.75 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(-0.5 * PI);
                    } else if humanoid_component.facing_direction > 0.5 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(-0.75 * PI);
                    } else if humanoid_component.facing_direction > 0.25 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(1. * PI);
                    } else if humanoid_component.facing_direction > 0. {
                        strafe_blend_position = strafe_blend_position.rotate(0.75 * PI);
                    } else if humanoid_component.facing_direction > -0.25 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(0.5 * PI);
                    } else if humanoid_component.facing_direction > -0.5 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(0.25 * PI);
                    } else if humanoid_component.facing_direction > -0.75 * PI {
                        strafe_blend_position = strafe_blend_position.rotate(0.);
                    } else if humanoid_component.facing_direction > -1. * PI {
                        strafe_blend_position = strafe_blend_position.rotate(-0.25 * PI);
                    }

                    animation_tree1_lower_body_strafe_blend_position.insert(
                        "blend_position".to_string(),
                        EntityUpdateData::Vec2(Vec2::new(
                            strafe_blend_position.x(),
                            strafe_blend_position.y(),
                        )),
                    );

                    animation_tree1_upper_body_strafe_blend_position.insert(
                        "blend_position".to_string(),
                        EntityUpdateData::Vec2(Vec2::new(
                            strafe_blend_position.x(),
                            strafe_blend_position.y(),
                        )),
                    );
                }
                CharacterAnimationState::Sprinting => {
                    upper_body_animation_state = "Idle Heightened".to_string();
                    lower_body_animation_state = "Sprinting".to_string();
                    upper_body_blend_amount = 0.;
                }
            }

            if humanoid_component.is_attacking {
                match active_item_entity {
                    Some(_entity) => match inventory_item_components_option
                        .unwrap()
                        .1
                        .combat_attack_animation
                    {
                        CombatAttackAnimation::OneHandedMeleePunch => {
                            if is_left_handed {
                                upper_body_animation_state = "Punching Left".to_string();
                            } else {
                                upper_body_animation_state = "Punching Right".to_string();
                            }
                        }
                        CombatAttackAnimation::PistolShot => {
                            if alt_attack_mode {
                                if is_left_handed {
                                    upper_body_animation_state = "Punching Left".to_string();
                                } else {
                                    upper_body_animation_state = "Punching Right".to_string();
                                }
                            }
                        }
                    },
                    None => {
                        if is_left_handed {
                            upper_body_animation_state = "Punching Left".to_string();
                        } else {
                            upper_body_animation_state = "Punching Right".to_string();
                        }
                    }
                }
            }
            animation_tree1_upper_blend.insert(
                "blend_amount".to_string(),
                EntityUpdateData::Float(upper_body_blend_amount),
            );
        } else {
            match humanoid_component.current_lower_animation_state {
                CharacterAnimationState::Idle => {
                    lower_body_animation_state = "Idle".to_string();
                    upper_body_animation_state = "Idle Heightened".to_string();
                }
                CharacterAnimationState::Jogging => {
                    lower_body_animation_state = "Jogging".to_string();
                    upper_body_animation_state = "Jogging".to_string();
                }
                CharacterAnimationState::Sprinting => {
                    lower_body_animation_state = "Sprinting".to_string();
                    upper_body_animation_state = "Idle Heightened".to_string();
                }
            }

            animation_tree1_upper_blend
                .insert("blend_amount".to_string(), EntityUpdateData::Float(0.));
        }

        let mut animation_tree1_upper_body_updates = HashMap::new();
        let mut animation_tree1_lower_body_updates = HashMap::new();

        animation_tree1_upper_body_updates.insert(
            "travel".to_string(),
            EntityUpdateData::String(upper_body_animation_state),
        );
        animation_tree1_lower_body_updates.insert(
            "travel".to_string(),
            EntityUpdateData::String(lower_body_animation_state),
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyState/playback/travel".to_string(),
            animation_tree1_upper_body_updates
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/mainBodyState/playback/travel"
                .to_string(),
            animation_tree1_lower_body_updates,
        );

        entity_updates_component.updates.insert(
            "Smoothing/pawn/humanMale/rig/animationTree1>>parameters/upperBodyBlend/blend_amount"
                .to_string(),
            animation_tree1_upper_blend,
        );

        entity_updates_component.updates.insert(
            lower_body_strafe_blendspace2d_path.clone(),
            animation_tree1_lower_body_strafe_blend_position,
        );

        if update_upper_body {
            entity_updates_component.updates.insert(
                upper_body_strafe_blendspace2d_path,
                animation_tree1_upper_body_strafe_blend_position,
            );
        }

        match showcase_component_option {
            Some(_showcase_component) => {}
            None => {
                let mut billboard_username_updates = HashMap::new();

                billboard_username_updates.insert(
                    "bbcode".to_string(),
                    EntityUpdateData::String(
                        "[color=white][center][b]".to_owned()
                            + &persistent_player_data_component.character_name
                            + "[/b][/center][/color]",
                    ),
                );

                match connected_player_component_option {
                    Some(connected_player_component) => {
                        entity_updates_component.excluded_handles.insert("Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText/VControl/name".to_string(), vec![connected_player_component.handle]);
                    }
                    None => {}
                }

                entity_updates_component.updates.insert(
                    "Smoothing/pawn/humanMale/textViewPortChat0/ViewPort/chatText/VControl/name"
                        .to_string(),
                    billboard_username_updates,
                );
            }
        }

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}
