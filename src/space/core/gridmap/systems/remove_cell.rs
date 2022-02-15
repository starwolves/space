use bevy::prelude::{EventReader, ResMut, Commands, Query};
use doryen_fov::FovAlgorithm;

use crate::space::{core::{pawn::components::{Senser, ConnectedPlayer}, gridmap::{events::RemoveCell, resources::{GridmapMain, GridmapDetails1, CellUpdate, CellData, StructureHealth, DoryenMap, to_doryen_coordinates, Vec2Int}}, networking::resources::GridMapType, atmospherics::{resources::{AtmosphericsResource, EffectType, AtmosEffect, CELCIUS_KELVIN_OFFSET}, functions::get_atmos_index}}};

use super::senser_update_fov::FOV_DISTANCE;

pub const VACUUM_ATMOSEFFECT : AtmosEffect = AtmosEffect {
    target_temperature: -270.45 + CELCIUS_KELVIN_OFFSET,
    temperature_speed: 10.,
    heater : false,

    target_amount: 0.,
    amount_speed: 10.,
    remover : true,
};

pub fn remove_cell(
    mut deconstruct_cell_events : EventReader<RemoveCell>,
    mut gridmap_main : ResMut<GridmapMain>,
    mut gridmap_details1 : ResMut<GridmapDetails1>,
    mut fov_map : ResMut<DoryenMap>,
    mut commands : Commands,
    mut sensers : Query<(&mut Senser, &ConnectedPlayer)>,
    mut atmospherics_resource : ResMut<AtmosphericsResource>,
) {

    for event in deconstruct_cell_events.iter() {

        match event.gridmap_type {
            GridMapType::Main => {
                
                let coords = to_doryen_coordinates(event.id.x, event.id.z);
                
                let mut atmospherics = atmospherics_resource.atmospherics.get_mut(get_atmos_index(Vec2Int{x:event.id.x,y:event.id.z})).unwrap();

                if event.id.y == 0 {
                    // Wall
                    let cell_entity = gridmap_main.data.get(&event.id).unwrap().entity.unwrap();
                    commands.entity(cell_entity).despawn();
                    fov_map.map.set_transparent(coords.0, coords.1, true);
                    atmospherics.blocked=false;
                } else {

                    let mut upper_id = event.id.clone();
                    upper_id.y=0;

                    // Add vacuum flag to atmos.
                    match gridmap_main.data.get(&upper_id) {
                        Some(_) => todo!(),
                        None => {
                            atmospherics.effects.insert(EffectType::Floorless, VACUUM_ATMOSEFFECT);
                        },
                    }
                    

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
            GridMapType::Details1 => {
                
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
