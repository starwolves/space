use std::{collections::BTreeMap, sync::Arc};

use bevy_log::warn;
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::{
            resources::SpawnData,
            spawn::{BaseEntityBundle, BaseEntitySummonable, NoEntityData},
        },
        examinable::components::{Examinable, RichName},
        health::components::Health,
        tab_actions::components::{TabAction, TabActions},
    },
    entities::air_locks::functions::{
        lock_closed_action, lock_open_action, toggle_open_action, unlock_action,
    },
};

use super::CounterWindowSummoner;

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

impl BaseEntitySummonable<NoEntityData> for CounterWindowSummoner {
    fn get_bundle(&self, spawn_data: &SpawnData, _entity_data: NoEntityData) -> BaseEntityBundle {
        let entity_name = spawn_data.entity_name.clone();
        let department_name;

        if entity_name == "securityCounterWindow" {
            department_name = "security";
        } else if entity_name == "bridgeCounterWindow" {
            department_name = "bridge";
        } else {
            warn!("Unrecognized counterwindow sub-type {}", entity_name);
            department_name = "ERR";
        }
        let mut examine_map = BTreeMap::new();

        examine_map.insert(
            0,
            "An airtight ".to_string()
                + department_name
                + " window. It will only grant access to those authorised to use it.",
        );
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );
        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: department_name.to_string() + " window",
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            tab_actions_option: Some(TabActions {
                tab_actions: vec![
                    TabAction {
                        id: "actions::counter_windows/toggleopen".to_string(),
                        text: "Toggle Open".to_string(),
                        tab_list_priority: 100,
                        prerequisite_check: Arc::new(toggle_open_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::counter_windows/lockopen".to_string(),
                        text: "Lock Open".to_string(),
                        tab_list_priority: 99,
                        prerequisite_check: Arc::new(lock_open_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::counter_windows/lockclosed".to_string(),
                        text: "Lock Closed".to_string(),
                        tab_list_priority: 98,
                        prerequisite_check: Arc::new(lock_closed_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::counter_windows/unlock".to_string(),
                        text: "Unlock".to_string(),
                        tab_list_priority: 97,
                        prerequisite_check: Arc::new(unlock_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                ],
            }),
            health: Health {
                is_combat_obstacle: true,
                is_laser_obstacle: false,
                is_reach_obstacle: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
