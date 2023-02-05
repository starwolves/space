use actions::core::{BuildingActions, ListActionDataRequests};
use bevy::prelude::{warn, Entity, EventReader, Query, Res, ResMut, Resource};
use entity::{
    examine::Examinable,
    health::HealthContainer,
    senser::{to_doryen_coordinates, Senser, SensingAbility},
};
use math::grid::Vec3Int;
use resources::grid::CellFace;
use text_api::core::{
    get_empty_cell_message, get_space_message, ASTRIX, ENGINEERING_TEXT_COLOR, EXAMINATION_EMPTY,
    FURTHER_ITALIC_FONT, HEALTHY_COLOR, UNHEALTHY_COLOR,
};

use crate::grid::{CellData, Gridmap};

/// Manage examining the gridmap.

pub(crate) fn examine_map(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    gridmap_main: Res<Gridmap>,
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

        let mut examine_text;

        let coords = to_doryen_coordinates(
            examine_event.gridmap_cell_id.x,
            examine_event.gridmap_cell_id.z,
        );
        if !examiner_senser_component.fov.is_in_fov(coords.0, coords.1) {
            examine_text = get_empty_cell_message();
        } else {
            match gridmap_main.get_cell(examine_event.gridmap_cell_id, examine_event.face.clone()) {
                Some(ship_cell) => {
                    examine_text = examine_ship_cell(&ship_cell, &gridmap_main);
                }
                None => {
                    examine_text = get_space_message();
                }
            };
        }

        examine_text = examine_text + "\n";

        examine_event.message = examine_event.message.clone() + &examine_text;
    }
}

/// Set examine action header name.

pub(crate) fn set_action_header_name(
    mut building_action_data: ResMut<BuildingActions>,
    examinables: Query<&Examinable>,
    gridmap_main: Res<Gridmap>,
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

                names = gridmap_main.main_text_names.clone();

                let item_id;

                match gridmap_main.get_cell(gridmap.id, gridmap.face) {
                    Some(data) => {
                        item_id = data.item_0;
                    }
                    None => {
                        warn!("Couldnt find item_id!");
                        continue;
                    }
                }

                action_data_request.set_id(names.get(&item_id.id).unwrap().get_name().to_string());
            }
        }
    }
}

/// Examine a ship cell's health.

pub(crate) fn examine_map_health(
    mut examine_map_events: ResMut<GridmapExamineMessages>,
    gridmap_main: Res<Gridmap>,
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

        gridmap_result =
            gridmap_main.get_cell(examine_event.gridmap_cell_id, examine_event.face.clone());

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
/// Examine map message event.
#[derive(Clone)]

pub struct InputExamineMap {
    pub handle: u64,
    pub entity: Entity,
    pub gridmap_cell_id: Vec3Int,
    pub face: CellFace,
    /// Map examine message being built and sent back to the player.
    pub message: String,
}

impl Default for InputExamineMap {
    fn default() -> Self {
        Self {
            handle: 0,
            entity: Entity::from_bits(0),
            gridmap_cell_id: Vec3Int::default(),
            message: ASTRIX.to_string(),
            face: CellFace::default(),
        }
    }
}

/// Stores examine messages being built this frame for gridmap examination.
#[derive(Default, Resource)]

pub struct GridmapExamineMessages {
    pub messages: Vec<InputExamineMap>,
}

use networking::server::NetworkingChatServerMessage;
use networking::server::OutgoingReliableServerMessage;
use text_api::core::END_ASTRIX;

use bevy::prelude::EventWriter;
/// Finalize examining the ship gridmap.

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
                            gridmap_cell_id: c.id,
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

use networking::server::IncomingReliableClientMessage;

use crate::net::GridmapClientMessage;
/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<GridmapClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_examine_map: EventWriter<InputExamineMap>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            GridmapClientMessage::ExamineMap(cell_id_x, cell_id_y, cell_id_z) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_examine_map.send(InputExamineMap {
                            handle: message.handle,
                            entity: *player_entity,
                            gridmap_cell_id: Vec3Int {
                                x: cell_id_x,
                                y: cell_id_y,
                                z: cell_id_z,
                            },
                            ..Default::default()
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                }
            }
        }
    }
}

/// Examine a ship/gridmap cell and add the text as a function.

pub fn examine_ship_cell(ship_cell: &CellData, gridmap_data: &Res<Gridmap>) -> String {
    let examine_text: &str;
    let mut message = "\n".to_owned();
    message = message
        + "[font="
        + FURTHER_ITALIC_FONT
        + "]"
        + "You examine the "
        + &gridmap_data
            .main_text_names
            .get(&ship_cell.item_0.id)
            .unwrap()
            .get_name()
        + ".[/font]\n";

    if ship_cell.item_0.id != 0 {
        examine_text = gridmap_data
            .main_text_examine_desc
            .get(&ship_cell.item_0.id)
            .unwrap();
    } else {
        examine_text = EXAMINATION_EMPTY;
    }

    message = message + "[font=" + FURTHER_ITALIC_FONT + "]" + examine_text + ".[/font]";

    message
}
