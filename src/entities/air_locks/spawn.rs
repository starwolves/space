use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bevy_ecs::{entity::Entity, system::Commands};
use bevy_hierarchy::BuildChildren;
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
        pawn::components::ShipAuthorizationEnum,
        physics::functions::{get_bit_masks, ColliderGroup},
        sensable::components::Sensable,
        tab_actions::components::{TabAction, TabActions},
    },
    entities::air_locks::components::AirLock,
};

use super::functions::{lock_closed_action, lock_open_action, toggle_open_action, unlock_action};

pub const DEFAULT_AIR_LOCK_Y: f32 = 1.;
pub struct AirlockBundle;

impl AirlockBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        correct_transform: bool,
        _pawn_data_option: Option<SpawnPawnData>,
        _held_data_option: Option<SpawnHeldData>,
        default_map_spawn: bool,
        properties: HashMap<String, ConsoleCommandVariantValues>,
    ) -> Entity {
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

        let description;
        let sub_name;

        if entity_name == "securityAirLock1" {
            sub_name = "security";
            description = "An air lock with ".to_string()
                + "security"
                + " department colors. It will only grant access to security personnel.";
        } else if entity_name == "bridgeAirLock" {
            sub_name = "bridge";
            description = "An air lock with ".to_string()
                + "bridge"
                + " department colors. It will only grant access to high ranked personnel.";
        } else if entity_name == "governmentAirLock" {
            sub_name = "government";

            description = "An air lock with ".to_string()
                + "government"
                + " department colors. It will only grant access to a select few.";
        } else if entity_name == "vacuumAirLock" {
            sub_name = "vacuum";
            description = "An air lock with ".to_string()
                + "danger markings"
                + ". On the other side is nothing but space.";
        } else {
            warn!("Unrecognized airlock sub-type {}", entity_name);
            return Entity::from_bits(0);
        }

        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, description);
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

        let mut t = entity_transform.clone();

        if correct_transform {
            t.translation.y = 0.;
        }

        builder
            .insert(RigidBody::Fixed)
            .insert(t)
            .with_children(|children| {
                children
                    .spawn()
                    .insert(Collider::cuboid(1., 1., 0.2))
                    .insert(Transform::from_translation(Vec3::new(
                        0.,
                        DEFAULT_AIR_LOCK_Y,
                        0.,
                    )))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(CollisionGroups::new(masks.0, masks.1));
            })
            .insert_bundle((
                Sensable::default(),
                AirLock {
                    access_permissions: vec![ShipAuthorizationEnum::Security],
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
                        name: sub_name.to_string() + " airlock",
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
