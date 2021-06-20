use bevy::{math::{Vec2, Vec3}, prelude::{Added, Commands, Entity, Query}};
use bevy_rapier3d::prelude::{ColliderBundle, ColliderShape, RigidBodyBundle, RigidBodyCcd, RigidBodyType};

use std::collections::HashMap;

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, human_character::{HumanCharacter, State as HumanState}, pawn::Pawn, persistent_player_data::PersistentPlayerData, player_input::PlayerInput, radio::{Radio, RadioChannel}, sensable::Sensable, space_access::SpaceAccess, spawning::Spawning, visible_checker::VisibleChecker, world_mode::{WorldMode,WorldModes}}, enums::{space_access_enum::SpaceAccessEnum, space_jobs::SpaceJobsEnum}, functions::transform_to_isometry::transform_to_isometry};


pub fn on_spawning(
    query : Query<(Entity, &Spawning, &PersistentPlayerData),Added<Spawning>>,
    mut commands : Commands
) {
    
    for (
        entity_id,
        spawning_component,
        persistent_player_data_component,
    ) in query.iter() {

        /*let rigid_body_component = RigidBodyBuilder::new_dynamic()
        .lock_rotations()
        .ccd_enabled(true)
        .position(transform_to_isometry(spawning_component.transform));*/

        let rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            position: transform_to_isometry(spawning_component.transform).into(),
            ccd: RigidBodyCcd {
                ccd_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        };

        /*let collider_component = ColliderBuilder::capsule_y(0.9, 0.25)
        .translation(0., 1.1, 0.);*/

        let collider_component = ColliderBundle {
            //shape: ColliderShape::capsule(0.0, 0.9, 0.25),
            shape: ColliderShape::capsule(
                Vec3::new(0.0,0.0,0.0).into(),
                Vec3::new(0.0,0.9,0.0).into(), 
                0.25
            ),
            position: Vec3::new(0., 1.1, 0.).into(),
            ..Default::default()
        };

        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());

        commands.entity(entity_id).insert_bundle((
            rigid_body_component,
            collider_component,
            Sensable{
                is_audible : false,
                is_light:false,
                sensed_by_cached:vec![],
                sensed_by:vec![],
                always_sensed : false
            },
            VisibleChecker,
            EntityData {
                entity_class : "entity".to_string(),
                entity_type : "humanMale".to_string(),
                entity_group: EntityGroup::Pawn
            },
            EntityUpdates{
                updates: entity_updates_map,
                changed_parameters: vec![],
                excluded_handles:HashMap::new(),
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
            },
            Pawn {
                name: persistent_player_data_component.character_name.clone(),
                job: SpaceJobsEnum::Security
            },
            SpaceAccess{
                access : vec![SpaceAccessEnum::Security]
            },
            CachedBroadcastTransform::new(),
            Radio {
                listen_access: vec![RadioChannel::Common, RadioChannel::Security],
                speak_access: vec![RadioChannel::Common, RadioChannel::Security],
            }
        )).remove::<Spawning>();

        

    }

    
    
}
