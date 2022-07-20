pub fn gridmap_updates(
    mut gridmap_main: ResMut<GridmapMain>,
    mut gridmap_details1: ResMut<GridmapDetails1>,
    sensers: Query<(Entity, &Senser, &ConnectedPlayer)>,
    mut net_gridmap_updates: EventWriter<NetGridmapUpdates>,
) {
    for (cell_id, cell_update) in gridmap_main.updates.iter_mut() {
        let cell_coords = to_doryen_coordinates(cell_id.x, cell_id.z);
        for (senser_entity, senser_component, connected_player_component) in sensers.iter() {
            if connected_player_component.connected
                && !cell_update.entities_received.contains(&senser_entity)
                && senser_component.fov.is_in_fov(cell_coords.0, cell_coords.1)
            {
                cell_update.entities_received.push(senser_entity);
                if cell_update.cell_data.item != -1 {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::AddCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            cell_update.cell_data.item,
                            cell_update.cell_data.orientation,
                            GridMapType::Main,
                        ),
                    });
                } else {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::RemoveCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            GridMapType::Main,
                        ),
                    });
                }
            }
        }
    }

    for (cell_id, cell_update) in gridmap_details1.updates.iter_mut() {
        let cell_coords = to_doryen_coordinates(cell_id.x, cell_id.z);

        for (senser_entity, senser_component, connected_player_component) in sensers.iter() {
            if connected_player_component.connected
                && !cell_update.entities_received.contains(&senser_entity)
                && senser_component.fov.is_in_fov(cell_coords.0, cell_coords.1)
            {
                cell_update.entities_received.push(senser_entity);
                if cell_update.cell_data.item != -1 {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::AddCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            cell_update.cell_data.item,
                            cell_update.cell_data.orientation,
                            GridMapType::Details1,
                        ),
                    });
                } else {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::RemoveCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            GridMapType::Details1,
                        ),
                    });
                }
            }
        }
    }
}

use bevy::{
    hierarchy::Children,
    prelude::{
        warn, Commands, Component, Entity, EventReader, EventWriter, Query, Res, ResMut, With,
    },
};

use shared::{
    chat::{ASTRIX, EXAMINATION_EMPTY, FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT},
    data::{ConnectedPlayer, Vec3Int},
    gridmap::{
        to_doryen_coordinates, CellData, GridMapType, GridmapData, GridmapDetails1, GridmapMain,
        RemoveCell,
    },
    health::{CellUpdate, StructureHealth},
    network::ReliableServerMessage,
    senser::Senser,
};

pub fn examine_ship_cell(
    ship_cell: &CellData,
    gridmap_type: &GridMapType,
    gridmap_data: &Res<GridmapData>,
) -> String {
    let examine_text: &str;
    let mut message = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + /*ASTRIX +*/ "\n";
    message = message
        + "[font="
        + FURTHER_ITALIC_FONT
        + "]"
        + "You examine the "
        + &gridmap_data
            .main_text_names
            .get(&ship_cell.item)
            .unwrap()
            .get_name()
        + ".\n";

    if ship_cell.item != -1 {
        match gridmap_type {
            GridMapType::Main => {
                examine_text = gridmap_data
                    .main_text_examine_desc
                    .get(&ship_cell.item)
                    .unwrap();
            }
            GridMapType::Details1 => {
                examine_text = gridmap_data
                    .details1_text_examine_desc
                    .get(&ship_cell.item)
                    .unwrap();
            }
        }
    } else {
        examine_text = EXAMINATION_EMPTY;
    }

    message = message + "[font=" + FURTHER_ITALIC_FONT + "]" + examine_text + ".[/font]";

    message
}

pub fn get_empty_cell_message() -> String {
    "[font=".to_owned() + FURTHER_NORMAL_FONT + "]" + ASTRIX + "\n" + EXAMINATION_EMPTY
}

use bevy_rapier3d::prelude::RigidBody;
use doryen_fov::FovAlgorithm;

pub fn remove_cell(
    mut deconstruct_cell_events: EventReader<RemoveCell>,
    mut gridmap_main: ResMut<GridmapMain>,
    mut gridmap_details1: ResMut<GridmapDetails1>,
    mut fov_map: ResMut<DoryenMap>,
    mut commands: Commands,
    mut sensers: Query<(&mut Senser, &ConnectedPlayer)>,
    rigid_bodies: Query<&Children, With<RigidBody>>,
) {
    for event in deconstruct_cell_events.iter() {
        match event.gridmap_type {
            GridMapType::Main => {
                let coords = to_doryen_coordinates(event.id.x, event.id.z);

                if event.id.y == 0 {
                    // Wall
                    let cell_entity = gridmap_main
                        .grid_data
                        .get(&event.id)
                        .unwrap()
                        .entity
                        .unwrap();

                    match rigid_bodies.get(cell_entity) {
                        Ok(children) => {
                            for child in children.iter() {
                                commands.entity(*child).despawn();
                            }
                        }
                        Err(_rr) => {
                            warn!("Couldnt find rigidbody beloning to cell!");
                        }
                    }

                    commands.entity(cell_entity).despawn();
                    fov_map.map.set_transparent(coords.0, coords.1, true);
                }

                match gridmap_details1.data.get(&event.id) {
                    Some(_cell_data) => {
                        let mut local_copy = event.cell_data.clone();
                        local_copy.item = -1;

                        gridmap_details1.updates.insert(
                            event.id,
                            CellUpdate {
                                entities_received: vec![],
                                cell_data: local_copy,
                            },
                        );
                    }
                    None => {}
                }

                for (mut senser_component, _connected_player_component) in sensers.iter_mut() {
                    if senser_component.fov.is_in_fov(coords.0, coords.1) {
                        senser_component.fov.clear_fov();
                        let coords = to_doryen_coordinates(
                            senser_component.cell_id.x,
                            senser_component.cell_id.y,
                        );
                        senser_component.fov.compute_fov(
                            &mut fov_map.map,
                            coords.0,
                            coords.1,
                            FOV_DISTANCE,
                            true,
                        );

                        gridmap_main.updates.insert(
                            event.id,
                            CellUpdate {
                                entities_received: vec![],
                                cell_data: event.cell_data.clone(),
                            },
                        );
                    }
                }

                gridmap_main.grid_data.remove(&event.id);
            }
            GridMapType::Details1 => {
                gridmap_details1.updates.insert(
                    event.id,
                    CellUpdate {
                        entities_received: vec![],
                        cell_data: CellData {
                            item: -1,
                            orientation: 0,
                            health: StructureHealth::default(),
                            entity: None,
                        },
                    },
                );
            }
        }
    }
}

use serde::Deserialize;

use super::{
    fov::{DoryenMap, FOV_DISTANCE},
    net::NetGridmapUpdates,
};

#[derive(Component)]
pub struct Cell {
    pub id: Vec3Int,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            id: Vec3Int { x: 0, y: 0, z: 0 },
        }
    }
}

#[derive(Deserialize)]
pub struct CellDataWID {
    pub id: String,
    pub item: String,
    pub orientation: i64,
}
