use bevy::prelude::{Commands, EventReader};

use networking::client::IncomingReliableServerMessage;

use crate::grid::cell_id_to_world;
use crate::grid::CellIndexes;
use bevy::prelude::info;
use bevy::prelude::warn;
use bevy::prelude::Res;
use bevy::prelude::StandardMaterial;
use bevy::prelude::Transform;
use bevy::prelude::{shape, Color, Mesh, PbrBundle};
use bevy::prelude::{Assets, ResMut};
use player::net::PlayerServerMessage;

use crate::grid::Gridmap;
/// Spawn 3D debug camera on boarding.

pub(crate) fn spawn_cubes(
    gridmap_main: Res<Gridmap>,
    mut commands: Commands,
    mut messages: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in messages.iter() {
        match message.message {
            PlayerServerMessage::Boarded => {
                let mut spawned_i = 0;
                let mut chunk_i = 0;
                for chunk_option in gridmap_main.grid_data.iter() {
                    match chunk_option {
                        Some(chunk) => {
                            let mut cell_i = 0;
                            for tile_option in chunk.cells.iter() {
                                match tile_option {
                                    Some(tile) => {
                                        match gridmap_main.main_id_name_map.get(&tile.item_0) {
                                            Some(gridmap_cell_name) => {
                                                if gridmap_cell_name.contains("Floor")
                                                    || gridmap_cell_name.contains("Wall")
                                                {
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
                                                    commands.spawn(PbrBundle {
                                                        mesh: meshes.add(Mesh::from(shape::Cube {
                                                            size: 2.0,
                                                        })),
                                                        material: materials
                                                            .add(Color::rgb(0.8, 0.7, 0.6).into()),
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
                info!("Spawned {} cubes.", spawned_i);
            }
            _ => {}
        }
    }
}
