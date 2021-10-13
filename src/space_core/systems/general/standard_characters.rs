use std::{f32::consts::PI};

use bevy::{core::Time, math::{Quat, Vec2, Vec3}, prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, warn}};
use bevy_rapier3d::{na::{UnitQuaternion}, prelude::{RigidBodyPosition, RigidBodyVelocity}, rapier::{ math::{Real, Vector}}};

use crate::space_core::{bundles::{footsteps_sprinting_sfx::FootstepsSprintingSfxBundle, footsteps_walking_sfx::FootstepsWalkingSfxBundle}, components::{examinable::Examinable, footsteps_sprinting::FootstepsSprinting, footsteps_walking::FootstepsWalking, inventory::Inventory, inventory_item::{CombatType, InventoryItem}, linked_footsteps_running::LinkedFootstepsSprinting, linked_footsteps_walking::LinkedFootstepsWalking, pawn::{FacingDirection, Pawn, facing_direction_to_direction}, player_input::PlayerInput, sensable::{Sensable}, standard_character::{CharacterAnimationState, StandardCharacter}, static_transform::StaticTransform}, events::{general::{attack::Attack, input_mouse_action::InputMouseAction, input_select_body_part::InputSelectBodyPart, input_toggle_auto_move::InputToggleAutoMove}, net::{net_unload_entity::NetUnloadEntity}}, functions::{converters::{isometry_to_transform::isometry_to_transform, transform_to_isometry::transform_to_isometry}}, resources::{handle_to_entity::HandleToEntity, y_axis_rotations::PlayerYAxisRotations}};



const JOG_SPEED : f32 = 13.;
const RUN_SPEED : f32 = 18.;
const MELEE_FISTS_REACH : f32 = 1.2;
const COMBAT_ROTATION_SPEED : f32 = 18.;

pub fn standard_characters(
    mut standard_character_query : Query<(
        Entity,
        &mut PlayerInput,
        &mut RigidBodyVelocity,
        &mut StandardCharacter,
        &mut RigidBodyPosition,
        Option<&LinkedFootstepsWalking>,
        Option<&LinkedFootstepsSprinting>,
        &mut Pawn,
        &Inventory,
    )>,
    inventory_items_query : Query<(&InventoryItem, &Examinable)>,
    mut footsteps_query : Query<(
        Option<&FootstepsWalking>,
        Option<&FootstepsSprinting>,
        &mut StaticTransform
    )>,
    mut sensable_entities : Query<&mut Sensable>,
    time: Res<Time>,
    movement_rotations: Res<PlayerYAxisRotations>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands : Commands,

    mut attack_event_writer : EventWriter<Attack>,

    tuple0 : (
        EventWriter<NetUnloadEntity>,
        EventReader<InputMouseAction>,
        EventReader<InputSelectBodyPart>,
        EventReader<InputToggleAutoMove>,
    ),

) {

    let (
        mut net_unload_entity,
        mut input_mouse_action_events,
        mut input_select_body_part,
        mut input_toggle_auto_move,
    ) = tuple0;

    for event in input_mouse_action_events.iter() {

        match standard_character_query.get_component_mut::<PlayerInput>(event.entity) {
            Ok(mut played_input_component) => {

                played_input_component.is_mouse_action_pressed = event.pressed;

            },
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of InputMouseAction.");
            },
        }

    }

    for event in input_select_body_part.iter() {

        match standard_character_query.get_component_mut::<PlayerInput>(event.entity) {
            Ok(mut player_input_component) => {
                player_input_component.targetted_limb = event.body_part.clone();
            },
            Err(_rr) => {warn!("Couldnt find PlayerInput entity for input_select_body_part");},
        }

    }

    for event in input_toggle_auto_move.iter() {

        match standard_character_query.get_component_mut::<PlayerInput>(event.entity) {
            Ok(mut player_input_component) => {
                player_input_component.auto_move_enabled = !player_input_component.auto_move_enabled;
            },
            Err(_rr) => {warn!("Couldnt find PlayerInput entity for input_toggle_auto_move");},
        }

    }

    for (
        standard_character_entity,
        mut player_input_component,
        mut rigid_body_velocity_component,
        mut standard_character_component,
        mut rigid_body_position_component,
        linked_footsteps_walking_option,
        linked_footsteps_sprinting_option,
        mut pawn_component,
        inventory_component,
    ) in standard_character_query.iter_mut() {


        if player_input_component.auto_move_enabled { 
            if player_input_component.movement_vector.length() > 0.1 {
                player_input_component.auto_move_direction = player_input_component.movement_vector.clone();
            }
        } else {
            player_input_component.auto_move_direction = Vec2::ZERO;
        }



        if standard_character_component.combat_mode == false {

            if player_input_component.is_mouse_action_pressed {
                player_input_component.is_mouse_action_pressed = false;
            }

        }

        let mut speed_factor = JOG_SPEED;

        if player_input_component.sprinting {
            speed_factor = RUN_SPEED;
        }

        let player_input_movement_vector;

        if player_input_component.auto_move_enabled && player_input_component.movement_vector.length() < 0.1 {
            if player_input_component.auto_move_direction.length () < 0.1 {
                player_input_movement_vector = facing_direction_to_direction(&pawn_component.facing_direction);
                player_input_component.auto_move_direction = player_input_movement_vector;
            } else {
                player_input_movement_vector = player_input_component.auto_move_direction;
            }
            
        } else {
            player_input_movement_vector = player_input_component.movement_vector;
        }

        if player_input_movement_vector.x.abs() == 1. && player_input_movement_vector.y.abs() == 1. {
            speed_factor*=0.665;
        }

        let rapier_vector : Vector<Real> = Vec3::new(
            player_input_movement_vector.x * -speed_factor,
            -1.0,
            player_input_movement_vector.y * speed_factor,
        ).into();


        let mut rigid_body_position = rigid_body_position_component.position.clone();

        let mut movement_index : usize = 0;

        let mut idle = false;

        let mut facing_direction = pawn_component.facing_direction.clone();

        let delta_time = time.delta();
        
        standard_character_component.next_attack_timer.tick(delta_time);
        let ready_to_attack_this_frame = standard_character_component.next_attack_timer.finished();

        


        // If combat mode, specific new rotation based on mouse direction.
        if standard_character_component.combat_mode &&  !player_input_component.sprinting{

            let active_slot = inventory_component.get_slot(&inventory_component.active_slot);

            let mut rotation_offset = - 0.1*PI;

            if &inventory_component.active_slot == "right_hand" {
                rotation_offset = 0.11*PI;
            }

            let mut inventory_item_component_option = None;

            let mut inventory_item_slot_name = "his fists".to_string();

            match active_slot.slot_item {
                Some(item_entity) => {
                    match inventory_items_query.get(item_entity) {
                        Ok((item_component, examinable_component)) => {
                            inventory_item_component_option = Some(item_component);
                            inventory_item_slot_name = examinable_component.name.clone();

                            match item_component.combat_standard_animation {
                                crate::space_core::components::inventory_item::CombatStandardAnimation::StandardStance => {},
                                crate::space_core::components::inventory_item::CombatStandardAnimation::PistolStance => {

                                    if player_input_movement_vector.x != 0. || player_input_movement_vector.y != 0. {
                                        rotation_offset = - 0.0675*PI;
                                    } else {
                                        rotation_offset = - 0.24*PI;
                                    }

                                    
                                },
                            }
                        },
                        Err(_rr)=>{
                            warn!("Couldn't find inventory_item belonging to used inventory slot of attack.");
                        },
                    }
                },
                None => {},
            }

            let end_rotation = Quat::from_axis_angle(
                Vec3::new(0.,1.,0.),
                -standard_character_component.facing_direction - 0.5*PI + rotation_offset,
            );

            let mut rigid_body_transform = isometry_to_transform(rigid_body_position_component.position);

            let slerp_rotation;

            if rigid_body_transform.rotation.dot(end_rotation) > 0. {
                slerp_rotation = rigid_body_transform.rotation.slerp(end_rotation, delta_time.as_secs_f32()*COMBAT_ROTATION_SPEED);
            } else {
                let start_rotation = -rigid_body_transform.rotation.clone();
                slerp_rotation = start_rotation.slerp(end_rotation, delta_time.as_secs_f32()*COMBAT_ROTATION_SPEED);
            }

            rigid_body_transform.rotation = slerp_rotation;

            rigid_body_position_component.position = transform_to_isometry(rigid_body_transform);


            
            let mut attacking_this_frame = false;

            if player_input_component.is_mouse_action_pressed {
                if ready_to_attack_this_frame {
                    attacking_this_frame=true;
                }
                if ready_to_attack_this_frame {
                    standard_character_component.next_attack_timer.reset()
                }
                if standard_character_component.next_attack_timer.paused() {
                    standard_character_component.next_attack_timer.unpause();
                    standard_character_component.next_attack_timer.reset();
                }
                if !standard_character_component.is_attacking {
                    standard_character_component.is_attacking=true;
                }

            } else {
                if standard_character_component.is_attacking {
                    standard_character_component.is_attacking=false;
                }

            }

            

            if attacking_this_frame {

                // Get used inventory item and attack mode enum. Then on match execute directPreciseRayCastMeleeAttack
                let mut combat_type = &CombatType::MeleeDirect;
                let mut combat_damage_model = &standard_character_component.default_melee_damage_model;
                let mut combat_sound_set = &standard_character_component.default_melee_sound_set;
                
                match inventory_item_component_option {
                    Some(inventory_item_component) => {
                        combat_type = &inventory_item_component.combat_type;
                        match combat_type {
                            CombatType::MeleeDirect => {
                                combat_damage_model = &inventory_item_component.combat_melee_damage_model;
                                combat_sound_set = &inventory_item_component.combat_melee_sound_set;
                            },
                            CombatType::Projectile(_projecttile_type) => {
                                combat_damage_model = &inventory_item_component.combat_projectile_damage_model.as_ref().unwrap();
                                combat_sound_set = &inventory_item_component.combat_projectile_sound_set.as_ref().unwrap();
                            },
                        }
                        
                    },
                    None => {},
                }
                    
                

                let mut angle = standard_character_component.facing_direction;

                if angle < 0. {
                    angle = -PI - angle;
                } else {
                    angle = PI - angle;
                }

                let sensable_component = sensable_entities.get_mut(standard_character_entity).unwrap();

                attack_event_writer.send(Attack {
                    attacker_entity: standard_character_entity,
                    weapon_entity: active_slot.slot_item,
                    weapon_name : inventory_item_slot_name,
                    attacker_position: Vec3::new(
                        rigid_body_position_component.position.translation.x, 
                        1.0,
                        rigid_body_position_component.position.translation.z,
                    ),
                    angle: angle,
                    damage_model: combat_damage_model.clone(),
                    range: MELEE_FISTS_REACH,
                    combat_type: combat_type.clone(),
                    targetted_limb : player_input_component.targetted_limb.clone(),
                    attacker_name : standard_character_component.character_name.clone(),
                    combat_sound_set : combat_sound_set.clone(),
                    attacker_sensed_by : sensable_component.sensed_by.clone(),
                    attacker_sensed_by_cached: sensable_component.sensed_by_cached.clone(),
                });

                

            }

        }
        

        // Moving up.
        if player_input_movement_vector.y == 1. && player_input_movement_vector.x == 0. {
            movement_index = 0;
            facing_direction = FacingDirection::Up;
        }
        // Moving down.
        else if player_input_movement_vector.y == -1. && player_input_movement_vector.x == 0. {
            movement_index = 4;
            facing_direction = FacingDirection::Down;
        }
        // Moving left.
        else if player_input_movement_vector.y == 0. && player_input_movement_vector.x == -1. {
            movement_index = 2;
            facing_direction = FacingDirection::Left;
        }
        // Moving right.
        else if player_input_movement_vector.y == 0. && player_input_movement_vector.x == 1. {
            movement_index = 6;
            facing_direction = FacingDirection::Right;
        }
        // Moving up left.
        else if player_input_movement_vector.y == 1. && player_input_movement_vector.x == -1. {
            movement_index = 1;
            facing_direction = FacingDirection::UpLeft;
        }
        // Moving up right.
        else if player_input_movement_vector.y == 1. && player_input_movement_vector.x == 1. {
            movement_index = 7;
            facing_direction = FacingDirection::UpRight;
        }
        // Moving down left.
        else if player_input_movement_vector.y == -1. && player_input_movement_vector.x == -1. {
            movement_index = 3;
            facing_direction = FacingDirection::DownRight;
        }
        // Moving down right.
        else if player_input_movement_vector.y == -1. && player_input_movement_vector.x == 1. {
            movement_index = 5;
            facing_direction = FacingDirection::DownLeft;
        }
        
        else if player_input_movement_vector.y == 0. && player_input_movement_vector.x == 0. {
            idle=true;
        }

        pawn_component.facing_direction = facing_direction;

        let current_linear_velocity : Vec3 = rigid_body_velocity_component.linvel.into();

        
        
        match (standard_character_component.combat_mode && idle && current_linear_velocity.length() < 0.05) || 
        (standard_character_component.combat_mode == false && idle)
        {
            true => {

                if matches!(standard_character_component.current_lower_animation_state, CharacterAnimationState::Jogging) {
                    standard_character_component.current_lower_animation_state = CharacterAnimationState::Idle;
                    // Despawn FootstepsWalkingSfx here.


                    match linked_footsteps_walking_option {
                        Some(linked_footsteps_walking_component) => {

                            let mut sensable_component = sensable_entities.get_mut(linked_footsteps_walking_component.entity).unwrap();

                            sensable_component.despawn(
                                linked_footsteps_walking_component.entity,
                                &mut net_unload_entity,
                                &handle_to_entity
                            );

                            commands.entity(standard_character_entity).remove::<LinkedFootstepsWalking>();

                            commands.entity(linked_footsteps_walking_component.entity).despawn();
                            
                            
                        }
                        None => {}
                    }
                   

                }

                if matches!(standard_character_component.current_lower_animation_state, CharacterAnimationState::Sprinting) {
                    standard_character_component.current_lower_animation_state = CharacterAnimationState::Idle;
                    // Despawn FootstepsSprintingSfx here.

                    match linked_footsteps_sprinting_option {
                        Some(linked_footsteps_sprinting_component) => {

                            let mut sensable_component = sensable_entities.get_mut(linked_footsteps_sprinting_component.entity).unwrap();

                            sensable_component.despawn(
                                linked_footsteps_sprinting_component.entity,
                                &mut net_unload_entity,
                                &handle_to_entity
                            );

                            commands.entity(standard_character_entity).remove::<LinkedFootstepsSprinting>();

                            commands.entity(linked_footsteps_sprinting_component.entity).despawn();
                            
                            
                        }
                        None => {}
                    }

                }


            }
            false => {

                if standard_character_component.combat_mode == false   ||  player_input_component.sprinting{
                    rigid_body_position.rotation = UnitQuaternion::from_quaternion(movement_rotations.rotations[movement_index]); 
                    rigid_body_position_component.position = rigid_body_position;
                }

                if !player_input_component.sprinting && matches!(standard_character_component.current_lower_animation_state, CharacterAnimationState::Jogging) == false {

                    if matches!(standard_character_component.current_lower_animation_state, CharacterAnimationState::Sprinting) {
                        match linked_footsteps_sprinting_option {
                            Some(linked_footsteps_sprinting_component) => {
    
                                let mut sensable_component = sensable_entities.get_mut(linked_footsteps_sprinting_component.entity).unwrap();
    
                                sensable_component.despawn(
                                    linked_footsteps_sprinting_component.entity,
                                    &mut net_unload_entity,
                                    &handle_to_entity
                                );
    
                                commands.entity(standard_character_entity).remove::<LinkedFootstepsSprinting>();
    
                                commands.entity(linked_footsteps_sprinting_component.entity).despawn();
                                
                                
                            }
                            None => {}
                        }
                    }

                    standard_character_component.current_lower_animation_state = CharacterAnimationState::Jogging;

                    // Spawn FootstepsWalkingSfx entity here.

                    let repeating_sfx_id = commands.spawn_bundle(FootstepsWalkingSfxBundle::new(isometry_to_transform(rigid_body_position))).id();
                    
                    commands.entity(standard_character_entity).insert(LinkedFootstepsWalking{
                        entity: repeating_sfx_id
                    });

                } else if !player_input_component.sprinting && matches!(standard_character_component.current_lower_animation_state, CharacterAnimationState::Jogging) {
                    // Update transform of our FootstepsWalkingSfx Entity here. (Should be moved to its own 2tick/s system eventually)

                    match linked_footsteps_walking_option {
                        Some(linked_footsteps_walking_component) => {

                            let linked_footsteps_walking = footsteps_query.get_mut(linked_footsteps_walking_component.entity);
                            match linked_footsteps_walking {
                                Ok((_footsteps_walking_component, _footsteps_sprinting_component, mut static_transform_component)) => {

                                    static_transform_component.transform = isometry_to_transform(rigid_body_position);

                                }
                                Err(err) => {
                                    warn!("linked_footsteps_walking err: {}", err);
                                }
                            }

                        }
                        None => {}
                    }

                } else if player_input_component.sprinting && matches!(standard_character_component.current_lower_animation_state, CharacterAnimationState::Sprinting) == false {

                    if matches!(standard_character_component.current_lower_animation_state, CharacterAnimationState::Jogging) {
                        match linked_footsteps_walking_option {
                            Some(linked_footsteps_walking_component) => {
    
                                let mut sensable_component = sensable_entities.get_mut(linked_footsteps_walking_component.entity).unwrap();
    
                                sensable_component.despawn(
                                    linked_footsteps_walking_component.entity,
                                    &mut net_unload_entity,
                                    &handle_to_entity
                                );
    
                                commands.entity(standard_character_entity).remove::<LinkedFootstepsWalking>();
    
                                commands.entity(linked_footsteps_walking_component.entity).despawn();
                                
                                
                            }
                            None => {}
                        }
                    }

                    standard_character_component.current_lower_animation_state = CharacterAnimationState::Sprinting;

                    // Spawn FootstepsWalkingSfx entity here.

                    let repeating_sfx_id = commands.spawn_bundle(FootstepsSprintingSfxBundle::new(isometry_to_transform(rigid_body_position))).id();
                    
                    commands.entity(standard_character_entity).insert(LinkedFootstepsSprinting{
                        entity: repeating_sfx_id
                    });

                } else if player_input_component.sprinting && matches!(standard_character_component.current_lower_animation_state, CharacterAnimationState::Sprinting) {
                    // Update transform of our FootstepsSprintingSfx Entity here. (Should be moved to its own 2tick/s system eventually)

                    match linked_footsteps_sprinting_option {
                        Some(linked_footsteps_sprinting_component) => {

                            let linked_footsteps_sprinting = footsteps_query.get_mut(linked_footsteps_sprinting_component.entity);
                            match linked_footsteps_sprinting {
                                Ok((_footsteps_walking_component, _footsteps_sprinting_component, mut static_transform_component)) => {

                                    static_transform_component.transform = isometry_to_transform(rigid_body_position);

                                }
                                Err(err) => {
                                    warn!("linked_footsteps_sprinting err: {}", err);
                                }
                            }

                        }
                        None => {}
                    }

                }

            }
        }
        
        if player_input_movement_vector.x != 0. || player_input_movement_vector.y != 0. {
            rigid_body_velocity_component.linvel = rapier_vector;
        }
        

    }

}
