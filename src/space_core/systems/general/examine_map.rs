use bevy::{math::Vec3, prelude::{EventReader, EventWriter, Query, Res}};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::{components::pawn::Pawn, events::{general::examine_map::ExamineMap, net::net_chat_message::NetChatMessage}, functions::gridmap::{examine_main_cell::{examine_ship_cell, get_empty_cell_message}, gridmap_functions::world_to_cell_id}, resources::{gridmap_details1::GridmapDetails1, gridmap_main::GridmapMain, handle_to_entity::HandleToEntity, network_messages::ReliableServerMessage, precalculated_fov_data::{Vec2Int, Vec3Int}, world_fov::WorldFOV}};

pub fn examine_map(
    mut examine_map_events : EventReader<ExamineMap>,
    mut net_new_chat_message_event : EventWriter<NetChatMessage>,
    gridmap_main : Res<GridmapMain>,
    gridmap_details1 : Res<GridmapDetails1>,
    world_fov : Res<WorldFOV>,
    handle_to_entity : Res<HandleToEntity>,
    pawns : Query<(&Pawn, &RigidBodyPosition)>,
) {

    for examine_event in examine_map_events.iter() {

        let player_entity = handle_to_entity.map.get(&examine_event.handle)
        .expect("examine_map.rs couldn't find player entity of examining player.");

        let player_world_position = pawns.get(*player_entity)
        .expect("examine_map.rs couldn't get player components of examining player.");

        let player_cell_id = world_to_cell_id(Vec3::new(
            player_world_position.1.position.translation.x,
            player_world_position.1.position.translation.y,
            player_world_position.1.position.translation.z)
        );

        let player_cell_id_2 = Vec2Int {
            x: player_cell_id.x,
            y: player_cell_id.z,
        };

        let relevant_fov_option =  world_fov.data.get(&player_cell_id_2);

        let relevant_fov;

        match relevant_fov_option {
            Some(fov) => {
                relevant_fov = fov;
            },
            None => {
                continue;
            },
        }

        let examine_text;

        if !relevant_fov.contains(&Vec2Int {
            x: examine_event.gridmap_cell_id.x,
            y: examine_event.gridmap_cell_id.z,
        }) {
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

        
    
        net_new_chat_message_event.send(NetChatMessage {
            handle: examine_event.handle,
            message: ReliableServerMessage::ChatMessage(examine_text),
        });


    }

    

}
