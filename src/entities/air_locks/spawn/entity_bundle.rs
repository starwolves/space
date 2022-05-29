use std::{collections::BTreeMap, sync::Arc};

use bevy_ecs::entity::Entity;
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::{components::EntityGroup, spawn::EntityBundle},
        examinable::components::{Examinable, RichName},
        health::components::Health,
        tab_actions::components::{TabAction, TabActions},
    },
    entities::air_locks::functions::{
        lock_closed_action, lock_open_action, toggle_open_action, unlock_action,
    },
};

pub fn entity_bundle(
    default_transform: Transform,
    default_map_spawn: bool,
    description: String,
    sub_name: String,
    entity_name: String,
    entity_id: Entity,
) -> EntityBundle {
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

    EntityBundle {
        default_transform,
        examinable: Examinable {
            name: RichName {
                name: sub_name.to_string() + " airlock",
                n: false,
                ..Default::default()
            },
            assigned_texts: examine_map,
            ..Default::default()
        },
        entity_name: entity_name.to_string(),
        entity_group: EntityGroup::AirLock,
        tab_actions_option: Some(TabActions {
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
        }),
        health: Health {
            is_combat_obstacle: true,
            is_reach_obstacle: true,
            ..Default::default()
        },
        default_map_spawn,
    }
}
