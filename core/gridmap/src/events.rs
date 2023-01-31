use bevy::{
    hierarchy::Children,
    prelude::{warn, Commands, Component, Entity, EventReader, Query, Res, ResMut, With},
};

use bevy_rapier3d::prelude::RigidBody;
use doryen_fov::FovAlgorithm;
use text_api::core::{EXAMINATION_EMPTY, FURTHER_ITALIC_FONT};

use entity::senser::{to_doryen_coordinates, Senser};
use math::grid::Vec3Int;
use serde::Deserialize;

use crate::grid::{CellData, CellUpdate, Gridmap, GridmapChunk, Orientation, RemoveCell};

use super::fov::{DoryenMap, FOV_DISTANCE};
use networking::server::ConnectedPlayer;

use crate::net::GridmapServerMessage;
use networking::server::OutgoingReliableServerMessage;

use bevy::prelude::EventWriter;

/// Manage gridmap update events such as adding and removing cells.

pub(crate) fn gridmap_updates_manager(
    mut gridmap_main: ResMut<Gridmap>,
    sensers: Query<(Entity, &Senser, &ConnectedPlayer)>,
    mut server: EventWriter<OutgoingReliableServerMessage<GridmapServerMessage>>,
) {
    for (cell_id, cell_update) in gridmap_main.updates.iter_mut() {
        let cell_coords = to_doryen_coordinates(cell_id.x, cell_id.z);
        for (senser_entity, senser_component, connected_player_component) in sensers.iter() {
            if connected_player_component.connected
                && !cell_update.entities_received.contains(&senser_entity)
                && senser_component.fov.is_in_fov(cell_coords.0, cell_coords.1)
            {
                cell_update.entities_received.push(senser_entity);
                if cell_update.cell_data.item_0 != 0 {
                    server.send(OutgoingReliableServerMessage {
                        handle: connected_player_component.handle,
                        message: GridmapServerMessage::AddCell(
                            cell_id.x,
                            cell_id.y,
                            cell_id.z,
                            cell_update.cell_data.item_0,
                            cell_update.cell_data.orientation.clone(),
                        ),
                    });
                } else {
                    server.send(OutgoingReliableServerMessage {
                        handle: connected_player_component.handle,
                        message: GridmapServerMessage::RemoveCell(cell_id.x, cell_id.y, cell_id.z),
                    });
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
            .get(&ship_cell.item_0)
            .unwrap()
            .get_name()
        + ".[/font]\n";

    if ship_cell.item_0 != 0 {
        examine_text = gridmap_data
            .main_text_examine_desc
            .get(&ship_cell.item_0)
            .unwrap();
    } else {
        examine_text = EXAMINATION_EMPTY;
    }

    message = message + "[font=" + FURTHER_ITALIC_FONT + "]" + examine_text + ".[/font]";

    message
}

/// Event to set a gridmap cell.
pub struct SetCell {
    pub id: Vec3Int,
    pub data: CellData,
}

/// Set a gridmap cell.
pub(crate) fn set_cell(mut events: EventReader<SetCell>, mut gridmap_main: ResMut<Gridmap>) {
    for event in events.iter() {
        match gridmap_main.get_indexes(event.id) {
            Some(indexes) => match gridmap_main.grid_data.get_mut(indexes.chunk) {
                Some(chunk_option) => {
                    match chunk_option {
                        Some(_) => {}
                        None => {
                            *chunk_option = Some(GridmapChunk::default());
                        }
                    }
                    match chunk_option {
                        Some(chunk) => {
                            chunk.cells[indexes.cell] = Some(event.data.clone());
                        }
                        None => {
                            warn!("No chunk option");
                            continue;
                        }
                    }
                }
                None => {
                    warn!("set_cell couldn't find chunk.");
                }
            },
            None => {
                warn!("set_cell couldn't get cell indexes.");
            }
        }
    }
}

/// Remove a ship/gridmap cell.
pub(crate) fn remove_cell(
    mut deconstruct_cell_events: EventReader<RemoveCell>,
    mut gridmap_main: ResMut<Gridmap>,
    mut fov_map: ResMut<DoryenMap>,
    mut commands: Commands,
    mut sensers: Query<(&mut Senser, &ConnectedPlayer)>,
    rigid_bodies: Query<&Children, With<RigidBody>>,
) {
    for event in deconstruct_cell_events.iter() {
        let coords = to_doryen_coordinates(event.id.x, event.id.z);

        if event.id.y == 0 {
            // Wall

            let cell_entity;
            match gridmap_main.get_cell(event.id) {
                Some(cell_data) => match cell_data.entity_option {
                    Some(ent) => {
                        cell_entity = ent;
                    }
                    None => {
                        warn!("remove_cell couldn't get entity.");
                        continue;
                    }
                },
                None => {
                    warn!("remove_cell couldn't find cell data.");
                    continue;
                }
            }

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

        for (mut senser_component, _connected_player_component) in sensers.iter_mut() {
            if senser_component.fov.is_in_fov(coords.0, coords.1) {
                senser_component.fov.clear_fov();
                let coords =
                    to_doryen_coordinates(senser_component.cell_id.x, senser_component.cell_id.y);
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

        let cell_indexes;

        match gridmap_main.get_indexes(event.id) {
            Some(i) => {
                cell_indexes = i;
            }
            None => {
                warn!("remove_cell couldn't get cell index.");
                continue;
            }
        }

        match gridmap_main.grid_data.get_mut(cell_indexes.chunk) {
            Some(chunk_option) => match chunk_option {
                Some(chunk) => {
                    chunk.cells[cell_indexes.cell] = None;
                }
                None => {
                    warn!("Chunk {} not found", cell_indexes.chunk);
                }
            },
            None => {
                warn!("Tried to update a non-existing chunk");
            }
        };
    }
}

/// Component that represents a cell.
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

/// Represents a cell with some additional data.
#[derive(Deserialize)]

pub(crate) struct CellDataWID {
    pub id: String,
    pub item: String,
    pub orientation: Orientation,
}
