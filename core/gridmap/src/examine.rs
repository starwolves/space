use actions::core::{BuildingActions, ListActionDataRequests};
use api::chat::{
    get_empty_cell_message, get_space_message, ENGINEERING_TEXT_COLOR, FURTHER_ITALIC_FONT,
    HEALTHY_COLOR, UNHEALTHY_COLOR,
};
use bevy::prelude::{warn, EventReader, Query, Res, ResMut};
use entity::{
    examine::{Examinable, GridmapExamineMessages},
    health::HealthContainer,
    senser::{to_doryen_coordinates, Senser, SensingAbility},
};
use networking::messages::{GridMapLayer, InputExamineMap};

use crate::{
    events::examine_ship_cell,
    grid::{GridmapData, GridmapDetails1, GridmapMain},
};

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
                    GridMapLayer::Main => {
                        names = gridmap_data.main_text_names.clone();
                        cell_data = gridmap_main.grid_data.clone();
                    }
                    GridMapLayer::Details1 => {
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

/// Examine a ship cell's health.
pub(crate) fn examine_map_health(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    senser_entities: Query<&Senser>,
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

        let coords = to_doryen_coordinates(
            examine_event.gridmap_cell_id.x,
            examine_event.gridmap_cell_id.z,
        );
        if examiner_senser_component.fov.is_in_fov(coords.0, coords.1) {
            match ship_cell_option {
                Some(ship_cell) => {
                    let mut message = "".to_string();

                    let structure_health;

                    match &ship_cell.health.health_container {
                        HealthContainer::Structure(t) => {
                            structure_health = t;
                        }
                        _ => {
                            continue;
                        }
                    }

                    if structure_health.brute < 25.
                        && structure_health.burn < 25.
                        && structure_health.toxin < 25.
                    {
                        message = message
                            + "[font="
                            + FURTHER_ITALIC_FONT
                            + "][color="
                            + HEALTHY_COLOR
                            + "]\nIt is in perfect shape.[/color][/font]";
                    } else {
                        if structure_health.brute > 75. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt is heavily damaged.[/color][/font]";
                        } else if structure_health.brute > 50. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt is damaged.[/color][/font]";
                        } else if structure_health.brute > 25. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt is slightly damaged.[/color][/font]";
                        }

                        if structure_health.burn > 75. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt suffers from heavy burn damage.[/color][/font]";
                        } else if structure_health.burn > 50. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt suffers burn damage.[/color][/font]";
                        } else if structure_health.burn > 25. {
                            message = message
                                + "[font="
                                + FURTHER_ITALIC_FONT
                                + "][color="
                                + UNHEALTHY_COLOR
                                + "]\nIt has slight burn damage.[/color][/font]";
                        }
                    }
                    examine_event.message = examine_event.message.clone() + &message + "\n";
                }
                None => {}
            }
        }
    }
}

/// Examine gridmap.
pub(crate) fn examine_map_abilities(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    senser_entities: Query<&Senser>,
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

        let mut examine_text = "".to_string();

        for sensing_ability in examiner_senser_component.sensing_abilities.iter() {
            match sensing_ability {
                SensingAbility::ShipEngineerKnowledge => {
                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + ENGINEERING_TEXT_COLOR
                        + "]"
                        + "\n"
                        + "Ship Engineer Knowledge: [/color]"
                        + "\n"
                        + "Reference shows coordinates ("
                        + &examine_event.gridmap_cell_id.x.to_string()
                        + " , "
                        + &examine_event.gridmap_cell_id.z.to_string()
                        + ")."
                        + "[/font]\n";
                }
                _ => (),
            }
        }
        examine_event.message = examine_event.message.clone() + &examine_text;
    }
}

pub fn finalize_grid_examine_input(
    mut gridmap_messages: ResMut<GridmapExamineMessages>,
    mut gridmap_examine_input: EventReader<InputExamineMap>,
) {
    for input_event in gridmap_examine_input.iter() {
        gridmap_messages.messages.push(input_event.clone());
    }
}
