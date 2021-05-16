use bevy::{math::Vec2, prelude::{Added, Commands, Entity, Query}};

use std::collections::HashMap;

use crate::space_core::{components::{entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, human_character::{HumanCharacter, State as HumanState}, player_input::PlayerInput, spawning::Spawning, visible::Visible, visible_checker::VisibleChecker, world_mode::{WorldMode,WorldModes}}, functions::transform_to_isometry::transform_to_isometry};

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

    for (
        entity_id,
        spawning_component
    ) in query.iter() {

        let rigid_body_component = RigidBodyBuilder::new_dynamic()
        .lock_rotations()
        .ccd_enabled(true)
        .position(transform_to_isometry(spawning_component.transform));

        let collider_component = ColliderBuilder::capsule_y(0.9, 0.25)
        .translation(0., -0.9, 0.);

        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());

        commands.entity(entity_id).insert_bundle((
            rigid_body_component,
            collider_component,
            Visible{
                is_light:false,
                sensed_by_cached:vec![],
                sensed_by:vec![]
            },
            VisibleChecker,
            EntityData {
                entity_class : "entity".to_string(),
                entity_type : "humanMale".to_string(),
                entity_group: EntityGroup::Pawn
            },
            EntityUpdates{
                updates: entity_updates_map
            },
            WorldMode {
                mode : WorldModes::Kinematic
            },
            PlayerInput{
                movement_vector : Vec2::ZERO,
                sprinting : false
            },
            HumanCharacter {
                state : HumanState::Idle
            }
        )).remove::<Spawning>();

        

    }

    
    
}
