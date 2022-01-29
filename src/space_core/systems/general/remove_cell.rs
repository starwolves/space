use bevy::prelude::{EventReader, ResMut, Commands, Query, EventWriter};
use doryen_fov::FovAlgorithm;

use crate::space_core::{events::{general::remove_cell::RemoveCell, net::net_remove_cell::NetRemoveCell}, resources::{gridmap_main::GridmapMain, doryen_fov::{DoryenMap, to_doryen_coordinates, Vec3Int}, network_messages::{ReliableServerMessage, GridMapType}, gridmap_details1::GridmapDetails1}, components::{senser::Senser, connected_player::ConnectedPlayer}};

use super::senser_update_fov::FOV_DISTANCE;

pub fn remove_cell(
    mut deconstruct_cell_events : EventReader<RemoveCell>,
    mut gridmap_main : ResMut<GridmapMain>,
    mut gridmap_details1 : ResMut<GridmapDetails1>,
    mut fov_map : ResMut<DoryenMap>,
    mut commands : Commands,
    mut sensers : Query<(&mut Senser, &ConnectedPlayer)>,
    mut net_remove_cell : EventWriter<NetRemoveCell>,
) {

    for event in deconstruct_cell_events.iter() {

        match event.gridmap_type {
            crate::space_core::resources::network_messages::GridMapType::Main => {
                
                let coords = to_doryen_coordinates(event.id.x, event.id.z);
                
                if event.id.y == 0 {
                    // Wall
                    let cell_entity = gridmap_main.data.get(&event.id).unwrap().entity.unwrap();
                    commands.entity(cell_entity).despawn();
                    fov_map.map.set_transparent(coords.0, coords.1, true);
                    
                }

                match gridmap_details1.data.get(&event.id) {
                    Some(_cell_data) => {
                        
                        remove_details1_cell(
                            &mut gridmap_details1,
                            &event.id,
                            &GridMapType::Details1,
                            &mut sensers,
                            &mut net_remove_cell,
                        );

                    },
                    None => {

                    },
                }

                for (mut senser_component, connected_player_component) in sensers.iter_mut() {

                    if senser_component.fov.is_in_fov(coords.0, coords.1) {

                        senser_component.fov.clear_fov();
                        let coords = to_doryen_coordinates(senser_component.cell_id.x, senser_component.cell_id.y);
                        senser_component.fov.compute_fov(&mut fov_map.map, coords.0, coords.1, FOV_DISTANCE, true);


                        net_remove_cell.send(NetRemoveCell {
                            handle: connected_player_component.handle,
                            message: ReliableServerMessage::RemoveCell(event.id.x, event.id.y, event.id.z, event.gridmap_type.clone()),
                        });

                    }

                }

                gridmap_main.data.remove(&event.id);


            },
            crate::space_core::resources::network_messages::GridMapType::Details1 => {
                
                remove_details1_cell(
                    &mut gridmap_details1,
                    &event.id,
                    &event.gridmap_type,
                    &mut sensers,
                    &mut net_remove_cell,
                );
                
            },
        }

    }

}


fn remove_details1_cell(
    gridmap_details1 : &mut ResMut<GridmapDetails1>,
    event_id : &Vec3Int,
    event_gridmap_type : &GridMapType,
    sensers : &mut Query<(&mut Senser, &ConnectedPlayer)>,
    net_remove_cell : &mut EventWriter<NetRemoveCell>,
) {

    gridmap_details1.data.remove(&event_id);

    let coords = to_doryen_coordinates(event_id.x, event_id.z);

    for (senser_component, connected_player_component) in sensers.iter_mut() {

        if senser_component.fov.is_in_fov(coords.0, coords.1) {

            net_remove_cell.send(NetRemoveCell {
                handle: connected_player_component.handle,
                message: ReliableServerMessage::RemoveCell(event_id.x, event_id.y, event_id.z, event_gridmap_type.clone()),
            });

        }

    }

}
