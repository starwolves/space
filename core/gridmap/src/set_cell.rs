use bevy::prelude::{warn, EventReader, ResMut};
use math::grid::Vec3Int;
use resources::grid::CellFace;

use crate::grid::{CellData, GridItems, Gridmap, GridmapChunk};

/// Event to set a gridmap cell.
pub struct SetCell {
    pub id: Vec3Int,
    pub data: CellData,
    pub face: CellFace,
}

/// Set a gridmap cell.
pub(crate) fn set_cell(mut events: EventReader<SetCell>, mut gridmap_main: ResMut<Gridmap>) {
    for event in events.iter() {
        let strict = gridmap_main.get_strict_cell(event.id, event.face.clone());

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
                                chunk.cells[indexes.cell] = Some(GridItems::default());
                            }

                            let mut y = chunk.cells.get_mut(indexes.cell);
                            let x = y.as_mut().unwrap();

                            match x {
                                Some(_) => {}
                                None => {
                                    **x = Some(GridItems::default());
                                }
                            }

                            let mut grid_items = x.as_mut().unwrap();

                            match strict.face {
                                crate::grid::StrictCellFace::FrontWall => {
                                    grid_items.front_wall = Some(event.data.clone());
                                }
                                crate::grid::StrictCellFace::RightWall => {
                                    grid_items.right_wall = Some(event.data.clone());
                                }
                                crate::grid::StrictCellFace::Floor => {
                                    grid_items.floor = Some(event.data.clone());
                                }
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
