use std::collections::HashMap;

use bevy::prelude::{Res, ResMut, info};

use crate::space_core::resources::{precalculated_fov_data::{PrecalculatedFOVData, Vec2Int}, world_fov::WorldFOV};

const INIT_FOV_SQUARE_SIZE : u32 = 260;

pub fn world_fov(
    mut world_fov : ResMut<WorldFOV>,
    precalculated_fov_data : Res<PrecalculatedFOVData>,
) {

    if world_fov.init == true {
        world_fov.init = false;

        // Hardcode map corners/limits for FOV calculation.

        let total_cells_amount = INIT_FOV_SQUARE_SIZE * INIT_FOV_SQUARE_SIZE;

        //let half_cells_amount = INIT_FOV_SQUARE_SIZE / 2;

        let mut iterative_vector = Vec2Int {
            x: 0,
            y: 0
        };

        let mut iterative_y : i32 = -1;

        for i in 0..total_cells_amount {

            let iterative_x;

            if i == 0 {
                // Can't % by 0.
                iterative_x = 0;
            } else {
                // Get the remainder of i divided by strip size.
                //info!("{}", i % INIT_FOV_SQUARE_SIZE);
                iterative_x = i % INIT_FOV_SQUARE_SIZE;

                if iterative_x == 0 {
                    iterative_y+=1;
                }

            }

            iterative_vector.x = iterative_x as i16;
            iterative_vector.y = iterative_y as i16;

            world_fov.to_be_recalculated.push(iterative_vector);


        }

        

    }

    let mut new_fov_data : HashMap<Vec2Int, Vec<Vec2Int>> = HashMap::new();

    for vector in &world_fov.to_be_recalculated {

        

        update_cell_fov(
            &mut new_fov_data,
            &precalculated_fov_data,
            vector,
        );

    }

    world_fov.to_be_recalculated = vec![];


    for (key, value) in new_fov_data {

        world_fov.data.insert(key, value);

    }
    

}


fn update_cell_fov (
    mut new_fov_data : &HashMap<Vec2Int, Vec<Vec2Int>>,
    precalculated_fov_data : &Res<PrecalculatedFOVData>,
    cell_id : &Vec2Int,
) {

    
    


}