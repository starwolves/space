use bevy::prelude::{Query, ResMut};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::{dynamics::RigidBodySet, math::{Real, Vector}}};

use crate::space_core::components::player_input::PlayerInput;

pub fn move_player_bodies(
    query : Query<(&PlayerInput, &RigidBodyHandleComponent)>,
    mut rigid_bodies: ResMut<RigidBodySet>
) {

    for (
        player_input_component,
        rigid_body_handle_component
    ) in query.iter() {

        let rigid_body = rigid_bodies.get_mut(rigid_body_handle_component.handle())
        .expect("move_player_bodies.rs rigidbody handle was not present in RigidBodySet resource.");

        let rapier_vector : Vector<Real> = Vector::new(
            player_input_component.movement_vector.x * 0.2,
            0.,
            player_input_component.movement_vector.y * 0.2,
        );

        //info!("Applying vector: {}", rapier_vector);

        rigid_body.apply_impulse(rapier_vector, true);

    }

}
