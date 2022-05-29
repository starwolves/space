use std::sync::Arc;

use bevy_ecs::entity::Entity;
use bevy_log::warn;
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{
        components::EntityGroup,
        resources::SpawnData,
        spawn::{base_entity_builder, BaseEntityData},
    },
    pawn::components::ShipAuthorizationEnum,
    rigid_body::spawn::{rigidbody_builder, RigidBodySpawnData},
    tab_actions::components::{TabAction, TabActions},
};

pub mod entity_bundle;
pub mod rigidbody_bundle;

use entity_bundle::entity_bundle;
use rigidbody_bundle::rigidbody_bundle;

use super::{
    components::AirLock,
    functions::{lock_closed_action, lock_open_action, toggle_open_action, unlock_action},
};

pub const DEFAULT_AIR_LOCK_Y: f32 = 1.;
pub struct AirlockBundle;

impl AirlockBundle {
    pub fn spawn(mut spawn_data: SpawnData) -> Entity {
        let description;
        let sub_name;

        if spawn_data.entity_name == "securityAirLock1" {
            sub_name = "security";
            description = "An air lock with ".to_string()
                + "security"
                + " department colors. It will only grant access to security personnel.";
        } else if spawn_data.entity_name == "bridgeAirLock" {
            sub_name = "bridge";
            description = "An air lock with ".to_string()
                + "bridge"
                + " department colors. It will only grant access to high ranked personnel.";
        } else if spawn_data.entity_name == "governmentAirLock" {
            sub_name = "government";

            description = "An air lock with ".to_string()
                + "government"
                + " department colors. It will only grant access to a select few.";
        } else if spawn_data.entity_name == "vacuumAirLock" {
            sub_name = "vacuum";
            description = "An air lock with ".to_string()
                + "danger markings"
                + ". On the other side is nothing but space.";
        } else {
            warn!("Unrecognized airlock sub-type {}", spawn_data.entity_name);
            return Entity::from_bits(0);
        }

        let entity = spawn_data.commands.spawn().id();

        let default_transform = Transform::identity();

        let rigidbody_bundle = rigidbody_bundle();
        let entity_bundle = entity_bundle(
            default_transform,
            spawn_data.default_map_spawn,
            description,
            sub_name.to_string(),
            spawn_data.entity_name,
            entity,
        );

        rigidbody_builder(
            &mut spawn_data.commands,
            entity,
            RigidBodySpawnData {
                rigidbody_dynamic: false,
                rigid_transform: spawn_data.entity_transform,
                entity_is_stored_item: spawn_data.held_data_option.is_some(),
                collider: rigidbody_bundle.collider,
                collider_transform: rigidbody_bundle.collider_transform,
                collider_friction: rigidbody_bundle.collider_friction,
                collision_events: true,
                ..Default::default()
            },
        );

        base_entity_builder(
            &mut spawn_data.commands,
            entity,
            BaseEntityData {
                dynamicbody: false,
                entity_type: entity_bundle.entity_name.clone(),
                examinable: entity_bundle.examinable,
                is_item_in_storage: spawn_data.held_data_option.is_some(),
                tab_actions_option: Some(TabActions {
                    tab_actions: vec![
                        TabAction {
                            id: "actions::air_locks/toggleopen".to_string(),
                            text: "Toggle Open".to_string(),
                            tab_list_priority: 100,
                            prerequisite_check: Arc::new(toggle_open_action),
                            belonging_entity: Some(entity),
                        },
                        TabAction {
                            id: "actions::air_locks/lockopen".to_string(),
                            text: "Lock Open".to_string(),
                            tab_list_priority: 99,
                            prerequisite_check: Arc::new(lock_open_action),
                            belonging_entity: Some(entity),
                        },
                        TabAction {
                            id: "actions::air_locks/lockclosed".to_string(),
                            text: "Lock Closed".to_string(),
                            tab_list_priority: 98,
                            prerequisite_check: Arc::new(lock_closed_action),
                            belonging_entity: Some(entity),
                        },
                        TabAction {
                            id: "actions::air_locks/unlock".to_string(),
                            text: "Unlock".to_string(),
                            tab_list_priority: 97,
                            prerequisite_check: Arc::new(unlock_action),
                            belonging_entity: Some(entity),
                        },
                    ],
                }),
                entity_group: EntityGroup::AirLock,
                ..Default::default()
            },
        );

        spawn_data.commands.entity(entity).insert(AirLock {
            access_permissions: vec![ShipAuthorizationEnum::Security],
            ..Default::default()
        });

        entity
    }
}
