use std::collections::BTreeMap;

use bevy_ecs::{
    entity::Entity,
    prelude::Added,
    system::{Query, ResMut},
};
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::components::{DefaultMapEntity, EntityData},
        examinable::components::{Examinable, RichName},
        gridmap::{
            functions::gridmap_functions::world_to_cell_id,
            resources::{EntityGridData, GridmapMain, Vec2Int},
        },
        map::resources::{MapData, GREEN_MAP_TILE_COUNTER},
    },
    entities::counter_windows::components::CounterWindow,
};

pub fn counter_window_default_map_added(
    mut default_counter_windows: Query<
        (
            Entity,
            &Transform,
            &DefaultMapEntity,
            &EntityData,
            &mut Examinable,
        ),
        Added<CounterWindow>,
    >,
    mut map_data: ResMut<MapData>,
    mut gridmap_main: ResMut<GridmapMain>,
) {
    for (
        counter_window_entity,
        rigid_body_position_component,
        _,
        entity_data_component,
        mut examinable_component,
    ) in default_counter_windows.iter_mut()
    {
        let cell_id = world_to_cell_id(rigid_body_position_component.translation.into());
        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        map_data.data.insert(cell_id2, GREEN_MAP_TILE_COUNTER);

        gridmap_main.entity_data.insert(
            cell_id,
            EntityGridData {
                entity: counter_window_entity,
                entity_name: entity_data_component.entity_name.to_string(),
            },
        );

        if entity_data_component.entity_name == "securityCounterWindow" {
            examinable_component.name = RichName {
                name: "security counter window".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(0, "An airtight security window. It will only grant access to those authorised to use it.".to_string());
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
            examinable_component.assigned_texts = examine_map;
        } else if entity_data_component.entity_name == "bridgeCounterWindow" {
            examinable_component.name = RichName {
                name: "bridge counter window".to_string(),
                n: false,
                ..Default::default()
            };
            let mut examine_map = BTreeMap::new();
            examine_map.insert(0, "An airtight bridge window. It will only grant access to those authorised to use it.".to_string());
            examine_map.insert(
                1,
                "[font=".to_string()
                    + FURTHER_ITALIC_FONT
                    + "][color="
                    + HEALTHY_COLOR
                    + "]It is fully operational.[/color][/font]",
            );
            examinable_component.assigned_texts = examine_map;
        }
    }
}
