use bevy::{prelude::{EventReader, EventWriter, Query, Res, warn}};

use crate::space_core::{components::senser::Senser, events::{general::examine_map::InputExamineMap, net::net_chat_message::NetChatMessage}, functions::gridmap::{examine_cell::{examine_ship_cell, get_empty_cell_message}}, resources::{doryen_fov::{to_doryen_coordinates}, gridmap_data::GridmapData, gridmap_details1::GridmapDetails1, gridmap_main::GridmapMain, network_messages::ReliableServerMessage}};

pub fn examine_map(
    mut examine_map_events : EventReader<InputExamineMap>,
    mut net_new_chat_message_event : EventWriter<NetChatMessage>,
    gridmap_main : Res<GridmapMain>,
    gridmap_details1 : Res<GridmapDetails1>,
    senser_entities : Query<&Senser>,
    gridmap_data : Res<GridmapData>,
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


        let examine_text;

        let coords = to_doryen_coordinates(examine_event.gridmap_cell_id.x, examine_event.gridmap_cell_id.z);
        if !examiner_senser_component.fov.is_in_fov(coords.0, coords.1) {
            examine_text = get_empty_cell_message();
        } else {

            let gridmap_type = &examine_event.gridmap_type;

            let gridmap_result;

            match examine_event.gridmap_type{
                crate::space_core::resources::network_messages::GridMapType::Main => {
                    gridmap_result = gridmap_main.data.get(&examine_event.gridmap_cell_id);
                },
                crate::space_core::resources::network_messages::GridMapType::Details1 => {
                    gridmap_result = gridmap_details1.data.get(&examine_event.gridmap_cell_id);
                },
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
                    examine_text = examine_ship_cell(ship_cell, gridmap_type,&gridmap_data);
                },
                None => {
                    examine_text = get_empty_cell_message();
                },
            }
        }

        net_new_chat_message_event.send(NetChatMessage {
            handle: examine_event.handle,
            message: ReliableServerMessage::ChatMessage(examine_text),
        });


    }

    

}
