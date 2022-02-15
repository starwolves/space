use bevy::prelude::{ResMut};

use crate::space::core::{gridmap::{resources::{FOV_MAP_WIDTH, Vec2Int}}, atmospherics::{resources::{AtmosphericsResource, Atmospherics}, functions::get_atmos_index}};

const TEMPERATURE_DIFFUSIVITY : f32 = 1.;
const AMOUNT_DIFFUSIVITY : f32 = 1.;

pub fn diffusion (

    mut atmospherics : ResMut<AtmosphericsResource>,

) {

    let default_x = FOV_MAP_WIDTH as i16 / 2;

    let mut current_cell_id = Vec2Int {
        x: -default_x-1,
        y: -default_x,
    };

    let vacuum_atmos = Atmospherics::default();

    // Takes about 1ms.
    for _i in 0..FOV_MAP_WIDTH*FOV_MAP_WIDTH {

        current_cell_id.x+=1;

        if current_cell_id.x > default_x {
            current_cell_id.x = -default_x;
            current_cell_id.y +=1;        
        }

        let current_cell_atmos = atmospherics.atmospherics.get(get_atmos_index(current_cell_id)).unwrap();

        if current_cell_atmos.blocked {
            continue;
        }

        let mut total_temperature = 0.;
        let mut total_amount = 0.;

        let mut non_blocking_adjacents : u8 = 0;

        for j in 0..4 {

            let mut adjacent_cell_id = current_cell_id.clone();

            if j == 0 {
                adjacent_cell_id.x+=1
            } else if j == 1 {
                adjacent_cell_id.x-=1
            } else if j == 2 {
                adjacent_cell_id.y+=1
            } else {
                adjacent_cell_id.y-=1
            }

            let out_of_range;

            if AtmosphericsResource::is_id_out_of_range(adjacent_cell_id) {
                out_of_range=true;
            } else {
                match atmospherics.atmospherics.get(get_atmos_index(adjacent_cell_id)) {
                    Some(a) => {
                        if !a.blocked {
                            non_blocking_adjacents+=1;
                            total_temperature+=a.temperature;
                            total_amount+=a.amount;
                        }
                        out_of_range=false;
                    },
                    None => {
                        out_of_range=true;
                    },
                }
            }

            if out_of_range {
                // Tile is outside of map range, permanent vacuum.
                total_temperature+=vacuum_atmos.temperature;
                total_amount+=vacuum_atmos.amount;
            }

            

        }

        if non_blocking_adjacents == 0 {
            continue;
        }

        //let new_temperature = total_temperature / non_blocking_adjacents as f32;
        //let new_amount = total_amount / non_blocking_adjacents as f32;

        let new_temperature = (current_cell_atmos.temperature + TEMPERATURE_DIFFUSIVITY * (total_temperature / non_blocking_adjacents as f32)) / (1. + TEMPERATURE_DIFFUSIVITY);
        let new_amount = (current_cell_atmos.amount + AMOUNT_DIFFUSIVITY * (total_amount / non_blocking_adjacents as f32)) / (1. + AMOUNT_DIFFUSIVITY);

        let current_cell_atmos = atmospherics.atmospherics.get_mut(get_atmos_index(current_cell_id)).unwrap();

        current_cell_atmos.temperature = new_temperature;
        current_cell_atmos.amount = new_amount;

    }

}
