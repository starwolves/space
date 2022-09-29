use actions::core::{BuildingActions, ListActionDataRequests};
use api::{
    chat::{get_empty_cell_message, get_space_message},
    gridmap::{
        to_doryen_coordinates, GridMapLayer, GridmapDetails1, GridmapExamineMessages, GridmapMain,
    },
    senser::Senser,
};
use bevy::prelude::{warn, Query, Res, ResMut};
use examinable::examine::Examinable;

use crate::{events::examine_ship_cell, grid::GridmapData};

/// Manage examining the gridmap.
pub(crate) fn examine_map(
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
                GridMapLayer::Main => {
                    gridmap_result = gridmap_main.grid_data.get(&examine_event.gridmap_cell_id);
                }
                GridMapLayer::Details1 => {
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

/// Set examine action header name.
pub(crate) fn set_action_header_name(
    mut building_action_data: ResMut<BuildingActions>,
    examinables: Query<&Examinable>,
    gridmap_data: Res<GridmapData>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    mut action_data_requests: ResMut<ListActionDataRequests>,
) {
    for building in building_action_data.list.iter_mut() {
        let action_data_request;

        match action_data_requests.list.get_mut(&building.incremented_i) {
            Some(a) => {
                action_data_request = a;
            }
            None => {
                continue;
            }
        }

        match building.target_entity_option {
            Some(e) => match examinables.get(e) {
                Ok(examinable_component) => {
                    action_data_request.set_id(examinable_component.name.get_name().to_string());
                }
                Err(_) => {
                    warn!("Entity had no examinable component.");
                }
            },
            None => {
                let gridmap = building.target_cell_option.clone().unwrap();

                let names;
                let cell_data;

                match gridmap.1 {
                    api::gridmap::GridMapLayer::Main => {
                        names = gridmap_data.main_text_names.clone();
                        cell_data = gridmap_main.grid_data.clone();
                    }
                    api::gridmap::GridMapLayer::Details1 => {
                        names = gridmap_data.details1_text_names.clone();
                        cell_data = gridmap_details1.grid_data.clone();
                    }
                }

                let item_id;

                match cell_data.get(&gridmap.0) {
                    Some(data) => {
                        item_id = data.item;
                    }
                    None => {
                        warn!("Couldnt find item_id!");
                        continue;
                    }
                }

                action_data_request.set_id(names.get(&item_id).unwrap().get_name().to_string());
            }
        }
    }
}
