pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

use bevy::prelude::{info, Res, ResMut};

use crate::space::core::{
    atmospherics::{
        functions::get_atmos_index,
        resources::{Atmospherics, AtmosphericsResource, DEFAULT_INTERNAL_AMOUNT},
    },
    gridmap::resources::{GridmapMain, Vec2Int, Vec3Int, FOV_MAP_WIDTH},
};

use super::gridmap::resources::GridmapData;

pub fn startup_atmospherics(
    gridmap_main: Res<GridmapMain>,
    mut atmospherics: ResMut<AtmosphericsResource>,
    gridmap_main_data: Res<GridmapData>,
) {
    // Setup atmospherics.
    let default_x = FOV_MAP_WIDTH as i16 / 2;
    let default_z = FOV_MAP_WIDTH as i16 / 2;

    let mut current_cell_id = Vec2Int {
        x: -default_x - 1,
        y: -default_z,
    };

    let mut vacuum_cells: u32 = 0;

    for _i in 0..FOV_MAP_WIDTH * FOV_MAP_WIDTH {
        current_cell_id.x += 1;

        if current_cell_id.x > default_x {
            current_cell_id.x = -default_x;
            current_cell_id.y += 1;
        }

        let blocked;
        let push_up;

        match gridmap_main.data.get(&Vec3Int {
            x: current_cell_id.x,
            y: 0,
            z: current_cell_id.y,
        }) {
            Some(cell_data) => {
                let properties = gridmap_main_data
                    .main_cell_properties
                    .get(&cell_data.item)
                    .unwrap();
                blocked = properties.atmospherics_blocker;
                push_up = properties.atmospherics_pushes_up;
            }
            None => {
                blocked = false;
                push_up = false;
            }
        }

        let internal;

        if !blocked {
            match gridmap_main.data.get(&Vec3Int {
                x: current_cell_id.x,
                y: -1,
                z: current_cell_id.y,
            }) {
                Some(_cell_data) => {
                    internal = true;
                }
                None => {
                    internal = false;
                }
            }
        } else {
            internal = false;
        }

        if internal {
            atmospherics.atmospherics[get_atmos_index(current_cell_id)] =
                Atmospherics::new_internal();
        } else {
            let flags = vec!["default_vacuum".to_string()];
            atmospherics.atmospherics[get_atmos_index(current_cell_id)] = Atmospherics {
                blocked,
                flags,
                forces_push_up: push_up,
                ..Default::default()
            };
            vacuum_cells += 1;
        }
    }

    let internal_cells_count = (FOV_MAP_WIDTH * FOV_MAP_WIDTH - vacuum_cells as usize) as f32;

    let internal_m3 = internal_cells_count / 2.;

    let internal_mol = internal_cells_count * DEFAULT_INTERNAL_AMOUNT;
    let internal_mega_mol = internal_mol * 1e-6;
    let internal_liter = internal_m3 * 1000.;
    let internal_kilo_liter = internal_liter * 0.001;

    let vacuum_m3 = vacuum_cells as f32 / 2.;
    let vacuum_km3 = vacuum_m3 * 0.001;

    info!(
        "Loaded {:.1}Mmol atmosphere into {:.1}kl ship surrounded by {:.1}dam3 vacuum.",
        internal_mega_mol, internal_kilo_liter, vacuum_km3
    );
}
