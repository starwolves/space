use std::collections::HashMap;

use bevy::prelude::{warn, EventReader, EventWriter, ResMut};
use entity::health::{Health, HealthContainer, HealthFlag, StructureHealth};

use crate::grid::{AddGroup, AddTile, CellItem, GridCell, Gridmap, GridmapChunk};

pub(crate) fn add_wall_tile(mut events: EventReader<AddTile>, mut gridmap_main: ResMut<Gridmap>) {
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

                            let cell_data = CellItem {
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
                                group_instance_id_option: add_tile_event.group_instance_id_option,
                            };

                            match strict.face {
                                crate::grid::StrictCellFace::FrontWall => {
                                    grid_items.front_wall = Some(cell_data.clone());
                                }
                                crate::grid::StrictCellFace::RightWall => {
                                    grid_items.right_wall = Some(cell_data.clone());
                                }
                                _ => {
                                    continue;
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

pub(crate) fn add_wall_group(
    mut events: EventReader<AddGroup>,
    mut gridmap_main: ResMut<Gridmap>,
    mut set_tile: EventWriter<AddTile>,
) {
    for add_group_event in events.iter() {
        let wall_id = *gridmap_main.main_name_id_map.get("generic_wall_1").unwrap();
        let group_instance_id = gridmap_main.group_instance_incremental;
        gridmap_main.group_instance_incremental += 1;
        set_tile.send(AddTile {
            id: add_group_event.id,
            tile_type: wall_id,
            orientation: add_group_event.orientation.clone(),
            face: add_group_event.face.clone(),
            group_instance_id_option: Some(group_instance_id),
        });
        let mut high_id = add_group_event.id.clone();
        high_id.y += 1;
        set_tile.send(AddTile {
            id: high_id,
            tile_type: wall_id,
            orientation: add_group_event.orientation.clone(),
            face: add_group_event.face.clone(),
            group_instance_id_option: Some(group_instance_id),
        });
    }
}
