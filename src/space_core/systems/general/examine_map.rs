use bevy::{prelude::{EventReader, EventWriter, Query, Res, warn}};

use crate::space_core::{components::senser::Senser, events::{general::examine_map::ExamineMap, net::net_chat_message::NetChatMessage}, functions::gridmap::{examine_main_cell::{examine_ship_cell, get_empty_cell_message}}, resources::{doryen_fov::{Vec3Int, to_doryen_coordinates}, gridmap_details1::GridmapDetails1, gridmap_main::GridmapMain, network_messages::ReliableServerMessage}};

pub fn examine_map(
    mut examine_map_events : EventReader<ExamineMap>,
    mut net_new_chat_message_event : EventWriter<NetChatMessage>,
    gridmap_main : Res<GridmapMain>,
    gridmap_details1 : Res<GridmapDetails1>,
    senser_entities : Query<&Senser>,
) {

    for examine_event in examine_map_events.iter() {

        let examiner_senser_component;

        match senser_entities.get(examine_event.entity) {
            Ok(examiner_senser) => {
                examiner_senser_component = examiner_senser;
            },
            Err(_rr) => {
                warn!("Couldn't find examiner entity in &Senser query.");
                continue;
            },
        }


        let mut examine_text;

        let coords = to_doryen_coordinates(examine_event.gridmap_cell_id.x, examine_event.gridmap_cell_id.z);
        if !examiner_senser_component.fov.is_in_fov(coords.0, coords.1) {
            examine_text = get_empty_cell_message();
        } else {

            let gridmap_type = &examine_event.gridmap_type;

            let mut gridmap_result;

            match examine_event.gridmap_type{
                crate::space_core::resources::network_messages::GridMapType::Main => {
                    gridmap_result = gridmap_main.data.get(&examine_event.gridmap_cell_id);
                },
                crate::space_core::resources::network_messages::GridMapType::Details1 => {
                    gridmap_result = gridmap_details1.data.get(&examine_event.gridmap_cell_id);
                },
            }

            if matches!(gridmap_result, None) {

                match examine_event.gridmap_type{
                    crate::space_core::resources::network_messages::GridMapType::Main => {
                        gridmap_result = gridmap_main.data.get(&Vec3Int {
                            x: examine_event.gridmap_cell_id.x,
                            y: examine_event.gridmap_cell_id.y-1,
                            z: examine_event.gridmap_cell_id.z,
                        });
                    },
                    crate::space_core::resources::network_messages::GridMapType::Details1 => {
                        gridmap_result = gridmap_details1.data.get(&Vec3Int {
                            x: examine_event.gridmap_cell_id.x,
                            y: examine_event.gridmap_cell_id.y-1,
                            z: examine_event.gridmap_cell_id.z,
                        });
                    },
                }

            }


            let ship_cell_option;

            match gridmap_result {
                Some(gridmap_cell) => {
                    ship_cell_option = Some(gridmap_cell)
                },
                None => {
                    ship_cell_option = None;
                },
            }


            match ship_cell_option {
                Some(ship_cell) => {
                    examine_text = examine_ship_cell(ship_cell, gridmap_type);
                },
                None => {
                    examine_text = get_empty_cell_message();
                },
            }
        }

        
        
        examine_text = examine_text + "\n";

        net_new_chat_message_event.send(NetChatMessage {
            handle: examine_event.handle,
            message: ReliableServerMessage::ChatMessage(examine_text),
        });


    }

    

}
