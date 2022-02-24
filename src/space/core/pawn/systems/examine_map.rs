use bevy::prelude::{warn, EventReader, EventWriter, Query, Res};

use crate::space::core::{
    atmospherics::{
        functions::get_atmos_index,
        resources::{AtmosphericsResource, CELCIUS_KELVIN_OFFSET},
    },
    gridmap::{
        functions::examine_cell::{
            examine_ship_cell, get_empty_cell_message, get_space_message, END_ASTRIX,
        },
        resources::{to_doryen_coordinates, GridmapData, GridmapDetails1, GridmapMain, Vec2Int},
    },
    networking::resources::{GridMapType, ReliableServerMessage},
    pawn::{
        components::Senser,
        events::{InputExamineMap, NetChatMessage},
        functions::new_chat_message::FURTHER_ITALIC_FONT,
    },
};

pub fn examine_map(
    mut examine_map_events: EventReader<InputExamineMap>,
    mut net_new_chat_message_event: EventWriter<NetChatMessage>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    senser_entities: Query<&Senser>,
    gridmap_data: Res<GridmapData>,
    atmospherics_resource: Res<AtmosphericsResource>,
) {
    for examine_event in examine_map_events.iter() {
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
                    gridmap_result = gridmap_details1.data.get(&examine_event.gridmap_cell_id);
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

        for sensing_ability in examiner_senser_component.sensing_abilities.iter() {
            match sensing_ability {
                crate::space::core::pawn::components::SensingAbility::Atmospherics => {
                    let id = Vec2Int {
                        x: examine_event.gridmap_cell_id.x,
                        y: examine_event.gridmap_cell_id.z,
                    };

                    if AtmosphericsResource::is_id_out_of_range(id) {
                        continue;
                    }

                    let atmospherics = atmospherics_resource
                        .atmospherics
                        .get(get_atmos_index(id))
                        .unwrap();

                    if atmospherics.blocked {
                        continue;
                    }

                    examine_text = examine_text
                        + "[font="
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + ATMOSPHERICS_TEXT_COLOR
                        + "]"
                        + "\n"
                        + "Atmospherics DataLink: [/color]"
                        + "\n"
                        + "Temperature: "
                        + &(atmospherics.temperature - CELCIUS_KELVIN_OFFSET)
                            .floor()
                            .to_string()
                        + " c\n"
                        + "Pressure: "
                        + &atmospherics.get_pressure().floor().to_string()
                        + " kpa"
                        + "[/font]";
                }
            }
        }

        examine_text = examine_text + END_ASTRIX;

        net_new_chat_message_event.send(NetChatMessage {
            handle: examine_event.handle,
            message: ReliableServerMessage::ChatMessage(examine_text),
        });
    }
}

pub const ATMOSPHERICS_TEXT_COLOR: &str = "#1797ff";
