use bevy::prelude::{Res, ResMut};

use crate::space_core::ecs::{gridmap::{resources::{FOV_MAP_WIDTH, Vec2Int, Vec3Int, GridmapMain}}, atmospherics::{resources::{AtmosphericsResource, Atmospherics}, functions::get_atmos_index}};

pub fn startup_init_atmospherics(
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
                ..Default::default()
            }
        }


    }

}
