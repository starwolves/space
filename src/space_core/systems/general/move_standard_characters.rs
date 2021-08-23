use bevy::{math::{Quat, Vec3}, prelude::{Commands, Entity, EventWriter, Query, Res, warn}};
use bevy_rapier3d::{na::{UnitQuaternion}, prelude::{RigidBodyForces, RigidBodyMassProps, RigidBodyPosition, RigidBodyVelocity}, rapier::{ math::{Real, Vector}}};

use crate::space_core::{bundles::{footsteps_sprinting_sfx::FootstepsSprintingSfxBundle, footsteps_walking_sfx::FootstepsWalkingSfxBundle}, components::{footsteps_sprinting::FootstepsSprinting, footsteps_walking::FootstepsWalking, linked_footsteps_running::LinkedFootstepsSprinting, linked_footsteps_walking::LinkedFootstepsWalking, pawn::{FacingDirection, Pawn}, player_input::PlayerInput, sensable::{Sensable}, standard_character::{CharacterAnimationState, StandardCharacter}, static_transform::StaticTransform}, events::net::net_unload_entity::NetUnloadEntity, functions::converters::{isometry_to_transform::isometry_to_transform, transform_to_isometry::transform_to_isometry}, resources::{handle_to_entity::HandleToEntity, y_axis_rotations::PlayerYAxisRotations}};



pub fn move_standard_characters(
    mut query : Query<(
        Entity,
        &PlayerInput,
        &mut RigidBodyPosition,
        &mut RigidBodyVelocity,
        &mut RigidBodyMassProps,
        &mut RigidBodyForces,
        &mut StandardCharacter,
        Option<&LinkedFootstepsWalking>,
        Option<&LinkedFootstepsSprinting>,
        &mut Pawn,
    )>,
    mut footsteps_query : Query<(
        &mut Sensable,
        Option<&FootstepsWalking>,
        Option<&FootstepsSprinting>,
        &mut StaticTransform
    )>,
    //time: Res<Time>,
    movement_rotations: Res<PlayerYAxisRotations>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands : Commands,
    mut net_unload_entity : EventWriter<NetUnloadEntity>
) {

    for (
        entity,
        player_input_component,
        mut rigid_body_position_component,
        mut rigid_body_velocity_component,
        mut _rigid_body_massprops_component,
        mut _rigid_body_force_component,
        mut standard_character_component,
        linked_footsteps_walking_option,
        linked_footsteps_sprinting_option,
        mut pawn_component,
    ) in query.iter_mut() {

        let mut speed_factor = 6.25;

        if player_input_component.sprinting {
            speed_factor = 14.;
        }

        if player_input_component.movement_vector.x.abs() == 1. && player_input_component.movement_vector.y.abs() == 1. {
            speed_factor*=0.665;
        }

        //speed_factor*=time.delta_seconds();

        let rapier_vector : Vector<Real> = Vec3::new(
            player_input_component.movement_vector.x * -speed_factor,
            -1.0,
            player_input_component.movement_vector.y * speed_factor,
        ).into();


        let mut rigid_body_position = rigid_body_position_component.position.clone();

        let mut movement_index : usize = 0;

        let mut idle = false;

        let mut facing_direction = pawn_component.facing_direction.clone();


        // If combat mode, specific new rotation based on mouse direction.
        if standard_character_component.combat_mode {

            let mut rigid_body_transform = isometry_to_transform(rigid_body_position_component.position);

            rigid_body_transform.rotation = Quat::from_axis_angle(Vec3::new(0.,1.,0.), standard_character_component.facing_direction);

            rigid_body_position_component.position = transform_to_isometry(rigid_body_transform);

        }
        

        // Moving up.
        if player_input_component.movement_vector.y == 1. && player_input_component.movement_vector.x == 0. {
            movement_index = 0;
            facing_direction = FacingDirection::Up;
        }
        // Moving down.
        else if player_input_component.movement_vector.y == -1. && player_input_component.movement_vector.x == 0. {
            movement_index = 4;
            facing_direction = FacingDirection::Down;
        }
        // Moving left.
        else if player_input_component.movement_vector.y == 0. && player_input_component.movement_vector.x == -1. {
            movement_index = 2;
            facing_direction = FacingDirection::Left;
        }
        // Moving right.
        else if player_input_component.movement_vector.y == 0. && player_input_component.movement_vector.x == 1. {
            movement_index = 6;
            facing_direction = FacingDirection::Right;
        }
        // Moving up left.
        else if player_input_component.movement_vector.y == 1. && player_input_component.movement_vector.x == -1. {
            movement_index = 1;
            facing_direction = FacingDirection::UpLeft;
        }
        // Moving up right.
        else if player_input_component.movement_vector.y == 1. && player_input_component.movement_vector.x == 1. {
            movement_index = 7;
            facing_direction = FacingDirection::UpRight;
        }
        // Moving down left.
        else if player_input_component.movement_vector.y == -1. && player_input_component.movement_vector.x == -1. {
            movement_index = 5;
            facing_direction = FacingDirection::DownLeft;
        }
        // Moving down right.
        else if player_input_component.movement_vector.y == -1. && player_input_component.movement_vector.x == 1. {
            movement_index = 3;
            facing_direction = FacingDirection::DownRight;
        }
        
        else if player_input_component.movement_vector.y == 0. && player_input_component.movement_vector.x == 0. {
            idle=true;
        }

        pawn_component.facing_direction = facing_direction;
        
        match idle {
            true => {

                if matches!(standard_character_component.current_animation_state, CharacterAnimationState::Walking) {
                    standard_character_component.current_animation_state = CharacterAnimationState::Idle;
                    // Despawn FootstepsWalkingSfx here.


                    match linked_footsteps_walking_option {
                        Some(linked_footsteps_walking_component) => {

                            let mut sensable_component = footsteps_query.get_component_mut::<Sensable>(linked_footsteps_walking_component.entity).unwrap();

                            sensable_component.despawn(
                                linked_footsteps_walking_component.entity,
                                &mut net_unload_entity,
                                &handle_to_entity
                            );

                            commands.entity(entity).remove::<LinkedFootstepsWalking>();

                            commands.entity(linked_footsteps_walking_component.entity).despawn();
                            
                            
                        }
                        None => {}
                    }
                   

                }

                if matches!(standard_character_component.current_animation_state, CharacterAnimationState::Sprinting) {
                    standard_character_component.current_animation_state = CharacterAnimationState::Idle;
                    // Despawn FootstepsSprintingSfx here.

                    match linked_footsteps_sprinting_option {
                        Some(linked_footsteps_sprinting_component) => {

                            let mut sensable_component = footsteps_query.get_component_mut::<Sensable>(linked_footsteps_sprinting_component.entity).unwrap();

                            sensable_component.despawn(
                                linked_footsteps_sprinting_component.entity,
                                &mut net_unload_entity,
                                &handle_to_entity
                            );

                            commands.entity(entity).remove::<LinkedFootstepsSprinting>();

                            commands.entity(linked_footsteps_sprinting_component.entity).despawn();
                            
                            
                        }
                        None => {}
                    }

                }


            }
            false => {

                if standard_character_component.combat_mode == false {
                    rigid_body_position.rotation = UnitQuaternion::from_quaternion(movement_rotations.rotations[movement_index]); 
                    rigid_body_position_component.position = rigid_body_position;
                }

                if !player_input_component.sprinting && matches!(standard_character_component.current_animation_state, CharacterAnimationState::Walking) == false {

                    if matches!(standard_character_component.current_animation_state, CharacterAnimationState::Sprinting) {
                        match linked_footsteps_sprinting_option {
                            Some(linked_footsteps_sprinting_component) => {
    
                                let mut sensable_component = footsteps_query.get_component_mut::<Sensable>(linked_footsteps_sprinting_component.entity).unwrap();
    
                                sensable_component.despawn(
                                    linked_footsteps_sprinting_component.entity,
                                    &mut net_unload_entity,
                                    &handle_to_entity
                                );
    
                                commands.entity(entity).remove::<LinkedFootstepsSprinting>();
    
                                commands.entity(linked_footsteps_sprinting_component.entity).despawn();
                                
                                
                            }
                            None => {}
                        }
                    }

                    standard_character_component.current_animation_state = CharacterAnimationState::Walking;

                    // Spawn FootstepsWalkingSfx entity here.

                    let repeating_sfx_id = commands.spawn_bundle(FootstepsWalkingSfxBundle::new(isometry_to_transform(rigid_body_position))).id();
                    
                    commands.entity(entity).insert(LinkedFootstepsWalking{
                        entity: repeating_sfx_id
                    });

                } else if !player_input_component.sprinting && matches!(standard_character_component.current_animation_state, CharacterAnimationState::Walking) {
                    // Update transform of our FootstepsWalkingSfx Entity here. (Should be moved to its own 2tick/s system eventually)

                    match linked_footsteps_walking_option {
                        Some(linked_footsteps_walking_component) => {

                            let linked_footsteps_walking = footsteps_query.get_mut(linked_footsteps_walking_component.entity);
                            match linked_footsteps_walking {
                                Ok((_sensable, _footsteps_walking_component, _footsteps_sprinting_component, mut static_transform_component)) => {

                                    static_transform_component.transform = isometry_to_transform(rigid_body_position);

                                }
                                Err(err) => {
                                    warn!("linked_footsteps_walking err: {}", err);
                                }
                            }

                        }
                        None => {}
                    }

                } else if player_input_component.sprinting && matches!(standard_character_component.current_animation_state, CharacterAnimationState::Sprinting) == false {

                    if matches!(standard_character_component.current_animation_state, CharacterAnimationState::Walking) {
                        match linked_footsteps_walking_option {
                            Some(linked_footsteps_walking_component) => {
    
                                let mut sensable_component = footsteps_query.get_component_mut::<Sensable>(linked_footsteps_walking_component.entity).unwrap();
    
                                sensable_component.despawn(
                                    linked_footsteps_walking_component.entity,
                                    &mut net_unload_entity,
                                    &handle_to_entity
                                );
    
                                commands.entity(entity).remove::<LinkedFootstepsWalking>();
    
                                commands.entity(linked_footsteps_walking_component.entity).despawn();
                                
                                
                            }
                            None => {}
                        }
                    }

                    standard_character_component.current_animation_state = CharacterAnimationState::Sprinting;

                    // Spawn FootstepsWalkingSfx entity here.

                    let repeating_sfx_id = commands.spawn_bundle(FootstepsSprintingSfxBundle::new(isometry_to_transform(rigid_body_position))).id();
                    
                    commands.entity(entity).insert(LinkedFootstepsSprinting{
                        entity: repeating_sfx_id
                    });

                } else if player_input_component.sprinting && matches!(standard_character_component.current_animation_state, CharacterAnimationState::Sprinting) {
                    // Update transform of our FootstepsSprintingSfx Entity here. (Should be moved to its own 2tick/s system eventually)

                    match linked_footsteps_sprinting_option {
                        Some(linked_footsteps_sprinting_component) => {

                            let linked_footsteps_sprinting = footsteps_query.get_mut(linked_footsteps_sprinting_component.entity);
                            match linked_footsteps_sprinting {
                                Ok((_sensable, _footsteps_walking_component, _footsteps_sprinting_component, mut static_transform_component)) => {

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
        
        rigid_body_velocity_component.linvel = rapier_vector;

    }

}
