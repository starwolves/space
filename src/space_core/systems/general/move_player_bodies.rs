use bevy::{core::Time, prelude::{Query, Res, ResMut, info}};
use bevy_rapier3d::{na::{UnitQuaternion}, physics::RigidBodyHandleComponent, rapier::{dynamics::RigidBodySet, math::{Real, Vector}}};

use crate::space_core::{components::{human_character::{HumanCharacter, State as HumanState}, player_input::PlayerInput}, resources::y_axis_rotations::PlayerYAxisRotations};



pub fn move_player_bodies(
    mut query : Query<(&PlayerInput, &RigidBodyHandleComponent, &mut HumanCharacter)>,
    time: Res<Time>,
    mut rigid_bodies: ResMut<RigidBodySet>,
    movement_rotations: Res<PlayerYAxisRotations>
) {

    for (
        player_input_component,
        rigid_body_handle_component,
        mut human_character_component
    ) in query.iter_mut() {

        let rigid_body = rigid_bodies.get_mut(rigid_body_handle_component.handle())
        .expect("move_player_bodies.rs rigidbody handle was not present in RigidBodySet resource.");

        let mut speed_factor = 500.;

        if player_input_component.movement_vector.x.abs() == 1. && player_input_component.movement_vector.y.abs() == 1. {
            // Can't invite the devil now can we.
            speed_factor*=0.665;
        }

        info!("{}",time.delta_seconds());

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
                }

            }
            false => {

                rigid_body_position.rotation = UnitQuaternion::from_quaternion(movement_rotations.rotations[movement_index]); 
                rigid_body.set_position(rigid_body_position, true);

                if matches!(human_character_component.state, HumanState::Walking) == false {
                    human_character_component.state = HumanState::Walking;
                }

            }
        }
        

        //info!("Applying vector: {}", rapier_vector);

        rigid_body.set_linvel(rapier_vector, true);

    }

}
