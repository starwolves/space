use std::{collections::BTreeMap, sync::Arc};

use bevy_log::warn;
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::{
            components::EntityGroup,
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

use super::AirlockSummoner;

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

impl BaseEntitySummonable<NoEntityData> for AirlockSummoner {
    fn get_bundle(&self, spawn_data: &SpawnData, _entity_data: NoEntityData) -> BaseEntityBundle {
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
            sub_name = "ERR";
            description = "ERR ".to_string();
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

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                name: RichName {
                    name: sub_name.to_string() + " airlock",
                    n: false,
                    ..Default::default()
                },
                assigned_texts: examine_map,
                ..Default::default()
            },
            entity_name: spawn_data.entity_name.to_string(),
            entity_group: EntityGroup::AirLock,
            tab_actions_option: Some(TabActions {
                tab_actions: vec![
                    TabAction {
                        id: "actions::air_locks/toggleopen".to_string(),
                        text: "Toggle Open".to_string(),
                        tab_list_priority: 100,
                        prerequisite_check: Arc::new(toggle_open_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::air_locks/lockopen".to_string(),
                        text: "Lock Open".to_string(),
                        tab_list_priority: 99,
                        prerequisite_check: Arc::new(lock_open_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::air_locks/lockclosed".to_string(),
                        text: "Lock Closed".to_string(),
                        tab_list_priority: 98,
                        prerequisite_check: Arc::new(lock_closed_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                    TabAction {
                        id: "actions::air_locks/unlock".to_string(),
                        text: "Unlock".to_string(),
                        tab_list_priority: 97,
                        prerequisite_check: Arc::new(unlock_action),
                        belonging_entity: Some(spawn_data.entity),
                    },
                ],
            }),
            health: Health {
                is_combat_obstacle: true,
                is_reach_obstacle: true,
                ..Default::default()
            },
            default_map_spawn: spawn_data.default_map_spawn,
        }
    }
}
