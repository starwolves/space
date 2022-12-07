use actions::core::{BuildingActions, ListActionDataRequests};
use bevy::prelude::{warn, Entity, EventReader, Query, Res, ResMut, Resource};
use entity::{
    examine::Examinable,
    health::HealthContainer,
    senser::{to_doryen_coordinates, Senser, SensingAbility},
};
use math::grid::Vec3Int;
use networking::server::GridMapLayer;
use text_api::core::{
    get_empty_cell_message, get_space_message, ASTRIX, ENGINEERING_TEXT_COLOR, FURTHER_ITALIC_FONT,
    HEALTHY_COLOR, UNHEALTHY_COLOR,
};

use crate::{
    events::examine_ship_cell,
    grid::{GridmapData, GridmapDetails1, GridmapMain},
};

/// Manage examining the gridmap.
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
pub fn finalize_grid_examine_input(
    mut gridmap_messages: ResMut<GridmapExamineMessages>,
    mut gridmap_examine_input: EventReader<InputExamineMap>,
) {
    for input_event in gridmap_examine_input.iter() {
        gridmap_messages.messages.push(input_event.clone());
    }
}
/// Examine map message event.
#[derive(Clone)]
#[cfg(feature = "server")]
pub struct InputExamineMap {
    pub handle: u64,
    pub entity: Entity,
    pub gridmap_type: GridMapLayer,
    pub gridmap_cell_id: Vec3Int,
    /// Map examine message being built and sent back to the player.
    pub message: String,
}
#[cfg(feature = "server")]
impl Default for InputExamineMap {
    fn default() -> Self {
        Self {
            handle: 0,
            entity: Entity::from_bits(0),
            gridmap_type: GridMapLayer::Main,
            gridmap_cell_id: Vec3Int::default(),
            message: ASTRIX.to_string(),
        }
    }
}

/// Stores examine messages being built this frame for gridmap examination.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub struct GridmapExamineMessages {
    pub messages: Vec<InputExamineMap>,
}

use networking::server::NetworkingChatServerMessage;
use networking::server::OutgoingReliableServerMessage;
use text_api::core::END_ASTRIX;

use bevy::prelude::EventWriter;
/// Finalize examining the ship gridmap.
#[cfg(feature = "server")]
pub(crate) fn finalize_examine_map(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    mut server: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
) {
    for event in examine_map_events.messages.iter_mut() {
        event.message = event.message.to_string() + END_ASTRIX;

        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: NetworkingChatServerMessage::ChatMessage(event.message.clone()),
        });
    }

    examine_map_events.messages.clear();
}

use actions::core::ActionRequests;
use networking::server::HandleToEntity;

/// Examine.
#[cfg(feature = "server")]
pub(crate) fn examine_grid(
    building_action_data: Res<BuildingActions>,
    mut examine_map_messages: ResMut<GridmapExamineMessages>,
    handle_to_entity: Res<HandleToEntity>,
    action_requests: Res<ActionRequests>,
) {
    for building in building_action_data.list.iter() {
        let building_action_id;
        match action_requests.list.get(&building.incremented_i) {
            Some(action_request) => {
                building_action_id = action_request.get_id().clone();
            }
            None => {
                continue;
            }
        }
        for action in building.actions.iter() {
            if action.is_approved()
                && action.data.id == "actions::pawn/examine"
                && action.data.id == building_action_id
            {
                match handle_to_entity.inv_map.get(&building.action_taker) {
                    Some(handle) => {
                        let c = building.target_cell_option.clone().unwrap();

                        examine_map_messages.messages.push(InputExamineMap {
                            handle: *handle,
                            entity: building.action_taker,
                            gridmap_type: c.1,
                            gridmap_cell_id: c.0,
                            ..Default::default()
                        });
                    }
                    None => {
                        warn!("Couldnt find examiner in handletoentity.");
                    }
                }
            }
        }
    }
}
