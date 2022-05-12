use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bevy_ecs::{entity::Entity, system::Commands};
use bevy_log::warn;
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionGroups, RigidBody};
use bevy_transform::components::Transform;

use crate::{
    core::{
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::{
            components::{DefaultMapEntity, EntityData, EntityGroup, EntityUpdates},
            resources::{SpawnHeldData, SpawnPawnData},
        },
        examinable::components::{Examinable, RichName},
        health::components::{Health, HealthFlag},
        networking::resources::ConsoleCommandVariantValues,
        pawn::components::SpaceAccessEnum,
        physics::functions::{get_bit_masks, ColliderGroup},
        sensable::components::Sensable,
        static_body::components::StaticTransform,
        tab_actions::components::{TabAction, TabActions},
    },
    entities::air_locks::components::AirLock,
};

use super::functions::{lock_closed_action, lock_open_action, toggle_open_action, unlock_action};

pub struct AirlockBundle;

impl AirlockBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        _correct_transform: bool,
        _pawn_data_option: Option<SpawnPawnData>,
        _held_data_option: Option<SpawnHeldData>,
        default_map_spawn: bool,
        properties: HashMap<String, ConsoleCommandVariantValues>,
    ) -> Entity {
        let static_transform_component = StaticTransform {
            transform: entity_transform,
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        let mut entity_name = "";

        match properties.get("entity_name").unwrap() {
            ConsoleCommandVariantValues::String(name) => {
                entity_name = name;
            }
            _ => {
                warn!("Incorrect entity_name type.");
            }
        }

        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "An air lock with bridge department colors. Access is only granted to high ranking staff."
                .to_string(),
        );
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );

        let mut health_flags = HashMap::new();

        health_flags.insert(0, HealthFlag::ArmourPlated);

        let mut builder = commands.spawn();
        let entity_id = builder.id();

        builder
            .insert(RigidBody::Fixed)
            .insert(Transform::from(entity_transform))
            .insert(Collider::cuboid(1., 1., 0.2))
            .insert(Transform::from_translation(Vec3::new(0., 1., 0.)))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(CollisionGroups::new(masks.0, masks.1))
            .insert_bundle((
                static_transform_component,
                Sensable::default(),
                AirLock {
                    access_permissions: vec![SpaceAccessEnum::Security],
                    ..Default::default()
                },
                EntityData {
                    entity_class: "entity".to_string(),
                    entity_name: entity_name.to_string(),
                    entity_group: EntityGroup::AirLock,
                },
                EntityUpdates::default(),
                Examinable {
                    name: RichName {
                        name: "bridge airlock".to_string(),
                        n: false,
                        ..Default::default()
                    },
                    assigned_texts: examine_map,
                    ..Default::default()
                },
                Health {
                    is_combat_obstacle: true,
                    is_reach_obstacle: true,
                    ..Default::default()
                },
                TabActions {
                    tab_actions: vec![
                        TabAction {
                            id: "actions::air_locks/toggleopen".to_string(),
                            text: "Toggle Open".to_string(),
                            tab_list_priority: 100,
                            prerequisite_check: Arc::new(toggle_open_action),
                            belonging_entity: Some(entity_id),
                        },
                        TabAction {
                            id: "actions::air_locks/lockopen".to_string(),
                            text: "Lock Open".to_string(),
                            tab_list_priority: 99,
                            prerequisite_check: Arc::new(lock_open_action),
                            belonging_entity: Some(entity_id),
                        },
                        TabAction {
                            id: "actions::air_locks/lockclosed".to_string(),
                            text: "Lock Closed".to_string(),
                            tab_list_priority: 98,
                            prerequisite_check: Arc::new(lock_closed_action),
                            belonging_entity: Some(entity_id),
                        },
                        TabAction {
                            id: "actions::air_locks/unlock".to_string(),
                            text: "Unlock".to_string(),
                            tab_list_priority: 97,
                            prerequisite_check: Arc::new(unlock_action),
                            belonging_entity: Some(entity_id),
                        },
                    ],
                },
            ));

        if default_map_spawn {
            builder.insert(DefaultMapEntity);
        }

        entity_id
    }
}
