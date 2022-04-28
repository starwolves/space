use std::collections::BTreeMap;

use bevy_ecs::{
    entity::Entity,
    prelude::Added,
    system::{Query, ResMut},
};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::{
    core::{
        atmospherics::{functions::get_atmos_index, resources::AtmosphericsResource},
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::components::EntityData,
        examinable::components::{Examinable, RichName},
        gridmap::{functions::gridmap_functions::world_to_cell_id, resources::Vec2Int},
    },
    entities::air_locks::components::AirLock,
};

pub fn air_lock_added(
    mut air_locks: Query<
        (
            Entity,
            &EntityData,
            &RigidBodyPositionComponent,
            &mut Examinable,
        ),
        Added<AirLock>,
    >,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for (
        _airlock_entity,
        entity_data_component,
        rigid_body_position_component,
        mut examinable_component,
    ) in air_locks.iter_mut()
    {
        let cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        if AtmosphericsResource::is_id_out_of_range(cell_id2) {
            continue;
        }
        let atmos_id = get_atmos_index(cell_id2);
        let atmospherics = atmospherics_resource
            .atmospherics
            .get_mut(atmos_id)
            .unwrap();

        atmospherics.blocked = true;

        if entity_data_component.entity_name == "bridgeAirLock" {
            examinable_component.name = RichName {
                name: "bridge airlock".to_string(),
                n: false,
                ..Default::default()
            };
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
            examinable_component.assigned_texts = examine_map;
        } else if entity_data_component.entity_name == "governmentAirLock" {
            examinable_component.name = RichName {
                name: "government airlock".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(
                0,
                "An air lock with government department colors. Access is only granted to a few elite crew members on-board."
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
        } else if entity_data_component.entity_name == "securityAirlock" {
            examinable_component.name = RichName {
                name: "security airlock".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(
                0,
                "An air lock with security department markings. It will only grant access to those authorised to use it."
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
        } else if entity_data_component.entity_name == "vacuumAirlock" {
            examinable_component.name = RichName {
                name: "vacuum airlock".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(
                0,
                "An air lock with vacuum warning colors. Opening this door will expose you to space."
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
        }
    }
}
