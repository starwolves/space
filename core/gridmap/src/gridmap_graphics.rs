use bevy::prelude::{Commands, EventReader};

use networking::client::IncomingReliableServerMessage;

use crate::grid::cell_id_to_world;
use crate::grid::GridmapData;
use bevy::prelude::info;
use bevy::prelude::warn;
use bevy::prelude::Res;
use bevy::prelude::StandardMaterial;
use bevy::prelude::Transform;
use bevy::prelude::{shape, Color, Mesh, PbrBundle};
use bevy::prelude::{Assets, ResMut};
use player::net::PlayerServerMessage;

use crate::grid::GridmapMain;
/// Spawn 3D debug camera on boarding.
#[cfg(feature = "client")]
pub(crate) fn spawn_cubes(
    gridmap_main: Res<GridmapMain>,
    gridmap_data: Res<GridmapData>,
    mut commands: Commands,
    mut messages: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for message in messages.iter() {
        match message.message {
            PlayerServerMessage::Boarded => {
                let mut spawned_i = 0;

                for (id, cell_data) in gridmap_main.grid_data.iter() {
                    match gridmap_data.main_id_name_map.get(&cell_data.item) {
                        Some(gridmap_cell_name) => {
                            if gridmap_cell_name.contains("Floor")
                                || gridmap_cell_name.contains("Wall")
                            {
                                spawned_i += 1;
                                commands.spawn(PbrBundle {
                                    mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
                                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                                    transform: Transform::from_translation(cell_id_to_world(*id)),
                                    ..Default::default()
                                });
                            }
                        }
                        None => {
                            warn!("Couldn't find cell data of item.");
                        }
                    }
                }
                info!("Spawned {} cubes.", spawned_i);
            }
            _ => {}
        }
    }
}
