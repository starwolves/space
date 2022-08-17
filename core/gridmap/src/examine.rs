use api::{
    chat::{get_empty_cell_message, get_space_message},
    gridmap::{
        to_doryen_coordinates, GridMapType, GridmapData, GridmapDetails1, GridmapExamineMessages,
        GridmapMain,
    },
    senser::Senser,
};
use bevy::prelude::{warn, Query, Res, ResMut};

use crate::events::examine_ship_cell;

pub fn examine_map(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    senser_entities: Query<&Senser>,
    gridmap_data: Res<GridmapData>,
) {
    for examine_event in examine_map_events.messages.iter_mut() {
        let examiner_senser_component;

        match senser_entities.get(examine_event.entity) {
            Ok(examiner_senser) => {
                examiner_senser_component = examiner_senser;
            }
            Err(_rr) => {
                warn!("Couldn't find examiner entity in &Senser query.");
                continue;
            }
        }

        let mut examine_text;

        let coords = to_doryen_coordinates(
            examine_event.gridmap_cell_id.x,
            examine_event.gridmap_cell_id.z,
        );
        if !examiner_senser_component.fov.is_in_fov(coords.0, coords.1) {
            examine_text = get_empty_cell_message();
        } else {
            let gridmap_type = &examine_event.gridmap_type;

            let gridmap_result;

            match examine_event.gridmap_type {
                GridMapType::Main => {
                    gridmap_result = gridmap_main.grid_data.get(&examine_event.gridmap_cell_id);
                }
                GridMapType::Details1 => {
                    gridmap_result = gridmap_details1
                        .grid_data
                        .get(&examine_event.gridmap_cell_id);
                }
            }

            let ship_cell_option;

            match gridmap_result {
                Some(gridmap_cell) => ship_cell_option = Some(gridmap_cell),
                None => {
                    ship_cell_option = None;
                }
            }

            match ship_cell_option {
                Some(ship_cell) => {
                    examine_text = examine_ship_cell(ship_cell, gridmap_type, &gridmap_data);
                }
                None => {
                    examine_text = get_space_message();
                }
            }
        }

        examine_text = examine_text + "\n";

        examine_event.message = examine_event.message.clone() + &examine_text;
    }
}
