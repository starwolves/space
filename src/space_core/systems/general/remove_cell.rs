use bevy::prelude::{EventReader, ResMut, Commands, Query};
use doryen_fov::FovAlgorithm;

use crate::space_core::{events::{general::remove_cell::RemoveCell}, resources::{gridmap_main::{GridmapMain, CellUpdate, CellData, StructureHealth}, doryen_fov::{DoryenMap, to_doryen_coordinates}, gridmap_details1::GridmapDetails1}, components::{senser::Senser, connected_player::ConnectedPlayer}};

use super::senser_update_fov::FOV_DISTANCE;

pub fn remove_cell(
    mut deconstruct_cell_events : EventReader<RemoveCell>,
    mut gridmap_main : ResMut<GridmapMain>,
    mut gridmap_details1 : ResMut<GridmapDetails1>,
    mut fov_map : ResMut<DoryenMap>,
    mut commands : Commands,
    mut sensers : Query<(&mut Senser, &ConnectedPlayer)>,
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
                        
                        let mut local_copy = event.cell_data.clone();
                        local_copy.item = -1;

                        gridmap_details1.updates.insert(event.id, CellUpdate {
                            entities_received: vec![],
                            cell_data: local_copy,
                        });

                    },
                    None => {

                    },
                }

                for (mut senser_component, _connected_player_component) in sensers.iter_mut() {

                    if senser_component.fov.is_in_fov(coords.0, coords.1) {

                        senser_component.fov.clear_fov();
                        let coords = to_doryen_coordinates(senser_component.cell_id.x, senser_component.cell_id.y);
                        senser_component.fov.compute_fov(&mut fov_map.map, coords.0, coords.1, FOV_DISTANCE, true);


                        gridmap_main.updates.insert(event.id, CellUpdate {
                            entities_received: vec![],
                            cell_data: event.cell_data.clone(),
                        });

                    }

                }

                gridmap_main.data.remove(&event.id);


            },
            crate::space_core::resources::network_messages::GridMapType::Details1 => {
                
                gridmap_details1.updates.insert(event.id, CellUpdate {
                    entities_received: vec![],
                    cell_data: CellData {
                        item: -1,
                        orientation: 0,
                        health: StructureHealth::default(),
                        entity: None,
                    },
                });
                
            },
        }

    }

}
