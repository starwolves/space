use std::collections::HashMap;

use bevy::prelude::{warn, EventReader, ResMut};
use entity::health::{Health, HealthContainer, HealthFlag, StructureHealth};
use math::grid::Vec3Int;
use resources::grid::CellFace;

use crate::grid::{CellItem, GridCell, Gridmap, GridmapChunk, Orientation};

/// Event to add a gridmap tile.
#[derive(Default)]
pub struct AddTile {
    pub id: Vec3Int,
    /// Id of tile type.
    pub tile_type: u16,
    /// Rotation.
    pub orientation: Orientation,
    pub face: CellFace,
}

pub(crate) fn add_floor_tile(mut events: EventReader<AddTile>, mut gridmap_main: ResMut<Gridmap>) {
    for add_tile_event in events.iter() {
        let strict = gridmap_main.get_strict_cell(add_tile_event.id, add_tile_event.face.clone());

        match gridmap_main.get_indexes(add_tile_event.id) {
            Some(indexes) => match gridmap_main.grid.get_mut(indexes.chunk) {
                Some(chunk_option) => {
                    match chunk_option {
                        Some(_) => {}
                        None => {
                            *chunk_option = Some(GridmapChunk::default());
                        }
                    }
                    match chunk_option {
                        Some(chunk) => {
                            let found;
                            match chunk.cells.get(indexes.cell) {
                                Some(_) => {
                                    found = true;
                                }
                                None => {
                                    found = false;
                                }
                            }

                            if !found {
                                chunk.cells[indexes.cell] = Some(GridCell::default());
                            }

                            let mut y = chunk.cells.get_mut(indexes.cell);
                            let x = y.as_mut().unwrap();

                            match x {
                                Some(_) => {}
                                None => {
                                    **x = Some(GridCell::default());
                                }
                            }

                            let mut grid_items = x.as_mut().unwrap();

                            let mut health_flags = HashMap::new();

                            health_flags.insert(0, HealthFlag::ArmourPlated);

                            match strict.face {
                                crate::grid::StrictCellFace::Floor => {
                                    grid_items.floor = Some(CellItem {
                                        tile_type: add_tile_event.tile_type,
                                        entity: None,
                                        group_entity: None,
                                        health: Health {
                                            health_flags: health_flags.clone(),
                                            health_container: HealthContainer::Structure(
                                                StructureHealth::default(),
                                            ),
                                            ..Default::default()
                                        },
                                        orientation: add_tile_event.orientation.clone(),
                                        group_id: None,
                                    });
                                }
                                _ => (),
                            }
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
