use bevy::{core::Time, prelude::{Commands, Entity, EventWriter, Query, Res, ResMut, warn}};
use bevy_rapier3d::{na::{UnitQuaternion}, physics::RigidBodyHandleComponent, rapier::{dynamics::RigidBodySet, math::{Real, Vector}}};

use crate::space_core::{bundles::footsteps_walking::FootstepsWalkingSfxBundle, components::{footsteps_walking::FootstepsWalking, human_character::{HumanCharacter, State as HumanState}, linked_footsteps_walking::LinkedFootstepsWalking, player_input::PlayerInput, sensable::{Sensable}, static_transform::StaticTransform}, events::net::net_unload_entity::NetUnloadEntity, functions::{isometry_to_transform::isometry_to_transform}, resources::{handle_to_entity::HandleToEntity, y_axis_rotations::PlayerYAxisRotations}};



pub fn move_player_bodies(
    mut query : Query<(
        Entity,
        &PlayerInput,
        &RigidBodyHandleComponent,
        &mut HumanCharacter,
        Option<&LinkedFootstepsWalking>,
    )>,
    mut footsteps_walking_query : Query<(
        &Sensable,
        &FootstepsWalking,
        &mut StaticTransform
    )>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    movement_rotations: Res<PlayerYAxisRotations>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands : Commands,
    mut net_unload_entity : EventWriter<NetUnloadEntity>
) {

    for (
        entity,
        player_input_component,
        rigid_body_handle_component,
        mut human_character_component,
        linked_footsteps_walking_option,
    ) in query.iter_mut() {

        let rigid_body = rigid_bodies.get_mut(rigid_body_handle_component.handle())
        .expect("move_player_bodies.rs rigidbody handle was not present in RigidBodySet resource.");

        let mut speed_factor = 500.;

        if player_input_component.movement_vector.x.abs() == 1. && player_input_component.movement_vector.y.abs() == 1. {
            speed_factor*=0.665;
        }

        speed_factor*=time.delta_seconds();

        let rapier_vector : Vector<Real> = Vector::new(
            player_input_component.movement_vector.x * -speed_factor,
            0.,
            player_input_component.movement_vector.y * speed_factor,
        );


        let mut rigid_body_position = rigid_body.position().clone();

        let mut movement_index : usize = 0;

        let mut idle = false;

        // Moving up.
        if player_input_component.movement_vector.y == 1. && player_input_component.movement_vector.x == 0. {
            movement_index = 0;
        }
        // Moving down.
        else if player_input_component.movement_vector.y == -1. && player_input_component.movement_vector.x == 0. {
            movement_index = 4;
        }
        // Moving left.
        else if player_input_component.movement_vector.y == 0. && player_input_component.movement_vector.x == -1. {
            movement_index = 2;
        }
        // Moving right.
        else if player_input_component.movement_vector.y == 0. && player_input_component.movement_vector.x == 1. {
            movement_index = 6;
        }
        // Moving up left.
        else if player_input_component.movement_vector.y == 1. && player_input_component.movement_vector.x == -1. {
            movement_index = 1;
        }
        // Moving up right.
        else if player_input_component.movement_vector.y == 1. && player_input_component.movement_vector.x == 1. {
            movement_index = 7;
        }
        // Moving down left.
        else if player_input_component.movement_vector.y == -1. && player_input_component.movement_vector.x == -1. {
            movement_index = 5;
        }
        // Moving down right.
        else if player_input_component.movement_vector.y == -1. && player_input_component.movement_vector.x == 1. {
            movement_index = 3;
        }
        
        else if player_input_component.movement_vector.y == 0. && player_input_component.movement_vector.x == 0. {
            idle=true;
        }
        
        match idle {
            true => {

                if matches!(human_character_component.state, HumanState::Idle) == false {
                    human_character_component.state = HumanState::Idle;
                    // Despawn FootstepsWalkingSfx here.


                    match linked_footsteps_walking_option {
                        Some(linked_footsteps_walking_component) => {

                            let sensable_component = footsteps_walking_query.get_component::<Sensable>(linked_footsteps_walking_component.entity).unwrap();

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

            }
            false => {

                rigid_body_position.rotation = UnitQuaternion::from_quaternion(movement_rotations.rotations[movement_index]); 
                rigid_body.set_position(rigid_body_position, true);

                if matches!(human_character_component.state, HumanState::Walking) == false {
                    human_character_component.state = HumanState::Walking;

                    // Spawn FootstepsWalkingSfx entity here.

                    let repeating_sfx_id = commands.spawn_bundle(FootstepsWalkingSfxBundle::new(isometry_to_transform(rigid_body_position))).id();
                    
                    commands.entity(entity).insert(LinkedFootstepsWalking{
                        entity: repeating_sfx_id
                    });

                } else {
                    // Update transform of our FootstepsWalkingSfx Entity here.

                    match linked_footsteps_walking_option {
                        Some(linked_footsteps_walking_component) => {

                            let linked_footsteps_walking = footsteps_walking_query.get_mut(linked_footsteps_walking_component.entity);
                            match linked_footsteps_walking {
                                Ok((_sensable, _footsteps_walking_component, mut static_transform_component)) => {

                                    static_transform_component.transform = isometry_to_transform(rigid_body_position);

                                }
                                Err(err) => {
                                    warn!("linked_footsteps_walking err: {}", err);
                                }
                            }

                        }
                        None => {}
                    }

                }

            }
        }
        

        //info!("Applying vector: {}", rapier_vector);
        rigid_body.set_linvel(rapier_vector, true);
        

    }

}
