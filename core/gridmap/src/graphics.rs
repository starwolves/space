use bevy::prelude::AssetServer;
use bevy::prelude::{Commands, EventReader};

use bevy::scene::SceneBundle;
use networking::client::IncomingReliableServerMessage;

use crate::grid::cell_id_to_world;
use crate::grid::CellIndexes;
use bevy::prelude::info;
use bevy::prelude::warn;
use bevy::prelude::Res;
use bevy::prelude::Transform;
use player::net::PlayerServerMessage;

use crate::grid::Gridmap;
/// Spawn 3D debug camera on boarding.

pub(crate) fn spawn_map_graphics(
    gridmap_main: Res<Gridmap>,
    mut commands: Commands,
    mut messages: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    assets: Res<AssetServer>,
) {
    for message in messages.iter() {
        match message.message {
            PlayerServerMessage::Boarded => {
                let floor_asset = assets.load("models/floor/floor.glb#Scene0");
                let wall_asset = assets.load("models/wall/wall.glb#Scene0");

                let mut spawned_i = 0;
                let mut chunk_i = 0;
                for chunk_option in gridmap_main.grid_data.iter() {
                    match chunk_option {
                        Some(chunk) => {
                            let mut cell_i = 0;
                            for tile_option in chunk.cells.iter() {
                                match tile_option {
                                    Some(tile) => {
                                        let mut cell_data_option = None;
                                        match &tile.floor {
                                            Some(data) => {
                                                cell_data_option = Some(data);
                                            }
                                            None => {}
                                        }
                                        match &tile.front_wall {
                                            Some(data) => {
                                                cell_data_option = Some(data);
                                            }
                                            None => {}
                                        }
                                        match &tile.right_wall {
                                            Some(data) => {
                                                cell_data_option = Some(data);
                                            }
                                            None => {}
                                        }
                                        let cell_data;
                                        match cell_data_option {
                                            Some(data) => {
                                                cell_data = data;
                                            }
                                            None => {
                                                continue;
                                            }
                                        }

                                        match gridmap_main
                                            .main_id_name_map
                                            .get(&cell_data.item_0.id)
                                        {
                                            Some(gridmap_cell_name) => {
                                                if gridmap_cell_name.contains("Floor")
                                                    || gridmap_cell_name.contains("Wall")
                                                {
                                                    let asset;

                                                    if gridmap_cell_name.contains("Floor") {
                                                        asset = floor_asset.clone();
                                                    } else {
                                                        asset = wall_asset.clone();
                                                    }

                                                    spawned_i += 1;
                                                    let id;
                                                    match gridmap_main.get_id(CellIndexes {
                                                        chunk: chunk_i,
                                                        cell: cell_i,
                                                    }) {
                                                        Some(i) => {
                                                            id = i;
                                                        }
                                                        None => {
                                                            warn!("Couldnt get id.");
                                                            continue;
                                                        }
                                                    }
                                                    commands.spawn(SceneBundle {
                                                        scene: asset,
                                                        transform: Transform::from_translation(
                                                            cell_id_to_world(id),
                                                        ),
                                                        ..Default::default()
                                                    });
                                                }
                                            }
                                            None => {
                                                warn!("Couldn't find cell data of item.");
                                            }
                                        }
                                    }
                                    None => {}
                                }
                                cell_i += 1;
                            }
                        }
                        None => {}
                    }
                    chunk_i += 1;
                }
                info!("Spawned {} mesh scene bundles.", spawned_i);
            }
            _ => {}
        }
    }
}
