use bevy::prelude::{Added, Commands, Entity, Query};

use crate::space_core::{components::{spawning::Spawning, visible::Visible, visible_checker::VisibleChecker}, functions::transform_to_isometry::transform_to_isometry};

use bevy_rapier3d::{
    rapier::{
        dynamics::{
            RigidBodyBuilder
        },
        geometry::{
            ColliderBuilder
        }
    }
};

pub fn on_spawning(
    query : Query<(Entity, &Spawning),Added<Spawning>>,
    mut commands : Commands
) {

    for (entity_id, spawning_component) in query.iter() {

        let rigid_body_component = RigidBodyBuilder::new_kinematic()
        .position(transform_to_isometry(spawning_component.transform));

        let collider_component = ColliderBuilder::capsule_y(1., 0.5);

        let visible_component = Visible{
            is_light:false,
            sensed_by_cached:vec![],
            sensed_by:vec![]
        };

        let visible_checker_component = VisibleChecker;

        commands.entity(entity_id).insert_bundle((
            rigid_body_component,
            collider_component,
            visible_component,
            visible_checker_component
        )).remove::<Spawning>();
    }


    
}
