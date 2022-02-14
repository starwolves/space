pub mod functions;
pub mod systems;
pub mod resources;


use bevy::prelude::{Res, ResMut, info};

use crate::space::core::{gridmap::{resources::{FOV_MAP_WIDTH, Vec2Int, Vec3Int, GridmapMain}}, atmospherics::{resources::{AtmosphericsResource, Atmospherics, DEFAULT_INTERNAL_AMOUNT}, functions::get_atmos_index}};

pub fn startup_atmospherics(
    gridmap_main : Res<GridmapMain>,
    mut atmospherics : ResMut<AtmosphericsResource>,
) {

    // Setup atmospherics.
    let default_x = FOV_MAP_WIDTH as i16 / 2;
    let default_z = FOV_MAP_WIDTH as i16 / 2;

    let mut current_cell_id = Vec2Int {
        x: -default_x-1,
        y: -default_z,
    };

    let mut vacuum_cells : u32 = 0;

    for _i in 0..FOV_MAP_WIDTH*FOV_MAP_WIDTH {

        current_cell_id.x+=1;

        if current_cell_id.x > default_x {
            current_cell_id.x = -default_x;
            current_cell_id.y +=1;        
        }

        let blocked;

        match gridmap_main.data.get(&Vec3Int{
            x: current_cell_id.x,
            y:0,
            z:current_cell_id.y
        }) {
            Some(_cell_data) => {
                blocked=true;
            },
            None => {
                blocked=false;
            },
        }

        let internal;

        if !blocked {

            match gridmap_main.data.get(&Vec3Int{
                x: current_cell_id.x,
                y:-1,
                z:current_cell_id.y
            }) {
                Some(_cell_data) => {
                    internal=true;
                },
                None => {
                    internal=false;
                },
            }

        } else {
            internal = false;
        }

        if internal {
            atmospherics.atmospherics[get_atmos_index(current_cell_id)] = Atmospherics::new_internal();
        } else {
            atmospherics.atmospherics[get_atmos_index(current_cell_id)] = Atmospherics {
                blocked,
                flags : vec!["default_vacuum".to_string()],
                ..Default::default()
            };
            vacuum_cells+=1;
        }


    }

    let internal_cells_count = (FOV_MAP_WIDTH*FOV_MAP_WIDTH-vacuum_cells as usize) as f32;

    let internal_m3 = internal_cells_count / 2.;
    
    let internal_mol = internal_cells_count * DEFAULT_INTERNAL_AMOUNT;
    let internal_mega_mol = internal_mol * 1e-6;
    let internal_liter = internal_m3 * 1000.;
    let internal_kilo_liter = internal_liter * 0.001;

    let vacuum_m3 = vacuum_cells as f32 / 2.;
    let vacuum_km3 = vacuum_m3 * 0.001;

    info!("Loaded {:.1}Mmol atmosphere into {:.1}kl of ship, simulating {:.1}dam3 of vacuum.", internal_mega_mol, internal_kilo_liter, vacuum_km3);

}
