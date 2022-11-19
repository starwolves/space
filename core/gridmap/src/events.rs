use bevy::{
    hierarchy::Children,
    prelude::{
        warn, Commands, Component, Entity, EventReader, EventWriter, Query, Res, ResMut, With,
    },
};

use bevy_rapier3d::prelude::RigidBody;
use doryen_fov::FovAlgorithm;
use text_api::core::{EXAMINATION_EMPTY, FURTHER_ITALIC_FONT};

use entity::{
    health::{Health, HealthContainer, StructureHealth},
    senser::{to_doryen_coordinates, Senser},
};
use math::grid::Vec3Int;
use networking::server::{GridMapLayer, ReliableServerMessage};
use serde::Deserialize;

use crate::grid::{CellData, CellUpdate, GridmapData, GridmapDetails1, GridmapMain, RemoveCell};

use super::{
    fov::{DoryenMap, FOV_DISTANCE},
    net::NetGridmapUpdates,
};
use networking::server::ConnectedPlayer;

/// Manage gridmap update events such as adding and removing cells.
#[cfg(feature = "server")]
pub(crate) fn gridmap_updates_manager(
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
                            GridMapLayer::Main,
                        ),
                    });
                } else {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::RemoveCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            GridMapLayer::Main,
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
                            GridMapLayer::Details1,
                        ),
                    });
                } else {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::RemoveCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            GridMapLayer::Details1,
                        ),
                    });
                }
            }
        }
    }
}

/// Examine a ship/gridmap cell and add the text as a function.
#[cfg(feature = "server")]
pub fn examine_ship_cell(
    ship_cell: &CellData,
    gridmap_type: &GridMapLayer,
    gridmap_data: &Res<GridmapData>,
) -> String {
    let examine_text: &str;
    let mut message = "\n".to_owned();
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
        + ".[/font]\n";

    if ship_cell.item != -1 {
        match gridmap_type {
            GridMapLayer::Main => {
                examine_text = gridmap_data
                    .main_text_examine_desc
                    .get(&ship_cell.item)
                    .unwrap();
            }
            GridMapLayer::Details1 => {
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

/// Remove a ship/gridmap cell.
#[cfg(feature = "server")]
pub(crate) fn remove_cell(
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
            GridMapLayer::Main => {
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

                match gridmap_details1.grid_data.get(&event.id) {
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
            GridMapLayer::Details1 => {
                gridmap_details1.updates.insert(
                    event.id,
                    CellUpdate {
                        entities_received: vec![],
                        cell_data: CellData {
                            item: -1,
                            orientation: 0,
                            health: Health {
                                health_container: HealthContainer::Structure(
                                    StructureHealth::default(),
                                ),
                                ..Default::default()
                            },
                            entity: None,
                        },
                    },
                );
            }
        }
    }
}

/// Component that represents a cell.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Cell {
    pub id: Vec3Int,
}

#[cfg(feature = "server")]
impl Default for Cell {
    fn default() -> Self {
        Self {
            id: Vec3Int { x: 0, y: 0, z: 0 },
        }
    }
}

/// Represents a cell with some additional data.
#[derive(Deserialize)]
#[cfg(feature = "server")]
pub(crate) struct CellDataWID {
    pub id: String,
    pub item: String,
    pub orientation: i64,
}
