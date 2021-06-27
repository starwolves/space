use std::collections::HashMap;

use bevy::{core::Time, math::{Vec3}, prelude::{Local, Res, ResMut, error, info}};
use bevy_rapier3d::prelude::{InteractionGroups, QueryPipeline, QueryPipelineColliderComponentsQuery, QueryPipelineColliderComponentsSet, Ray};

use crate::space_core::{functions::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, gridmap_functions::cell_id_to_world}, resources::{gridmap_main::{CellData, GridmapMain}, non_blocking_cells_list::NonBlockingCellsList, precalculated_fov_data::{PrecalculatedFOVData, Vec2Int, Vec3Int}, world_fov::WorldFOV}};

const INIT_FOV_SQUARE_SIZE : u32 = 260;
const VIEW_DISTANCE : i16 = 23;

const MAX_CELLS_PER_TICK : u8 = 10;

struct UnfilledCorner {
    pub empty_offset_left : Vec2Int,
    pub empty_offset_right : Vec2Int,
    pub blocker_offset : Vec2Int,
    pub to_be_shadowed_cell_if_eyes_left : Vec2Int,
    pub to_be_shadowed_cell_if_eyes_right : Vec2Int,
}


#[derive(Default)]
pub struct StartupProcessed {
    pub value : bool
}

pub fn world_fov(
    
    mut startup_processed : Local<StartupProcessed>,
    time: Res<Time>, 
    mut world_fov : ResMut<WorldFOV>,
    gridmap_main : Res<GridmapMain>,
    precalculated_fov_data : Res<PrecalculatedFOVData>,
    non_blocking_cells_list : Res<NonBlockingCellsList>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
) {

    let mut init_load = false;

    if world_fov.init == true {
        // Wait for physics engine to process all the new map cells, takes a short while.
        // If FOV bugs sometimes at startup calculations it may be because this value isnt high enough.
        if time.time_since_startup().as_millis() < 2000 {
            return;
        }
        init_load=true;
        world_fov.init = false;

        // Hardcode map corners/limits for FOV calculation.

        let total_cells_amount = INIT_FOV_SQUARE_SIZE * INIT_FOV_SQUARE_SIZE;

        let mut iterative_vector = Vec2Int {
            x: 0,
            y: 0
        };

        let square_half_size = INIT_FOV_SQUARE_SIZE / 2;

        let mut iterative_y : i16 = -1*square_half_size as i16;
        let mut iterative_x : i16 = -1*square_half_size as i16;

        iterative_x-=1;

        for _i in 0..total_cells_amount {

            iterative_x+=1;

            if iterative_x > square_half_size as i16 {
                iterative_x = -1*square_half_size as i16;
                iterative_y+=1;
            }

            iterative_vector.x = iterative_x as i16;
            iterative_vector.y = iterative_y as i16;

            world_fov.to_be_recalculated.push(iterative_vector);


        }

        
    }

    if world_fov.to_be_recalculated.len() == 0 && world_fov.to_be_recalculated_priority.len() == 0 {
        if !startup_processed.value {
            startup_processed.value=true;
            info!("Finished building FOV!");
        }
        return;
    }


    let mut new_fov_data : HashMap<Vec2Int, Vec<Vec2Int>> = HashMap::new();
    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    if init_load && world_fov.blocking_load_at_init {

        // Blockingly build FOV at startup.

        startup_processed.value = true;

        let start_size = world_fov.to_be_recalculated.len();
        let mut i = 0;
        let mut j =0 ;
        let frac_value = 0.2;
        let frac_size = (start_size as f32 * frac_value) as usize;

        for vector in &world_fov.to_be_recalculated {
            if i == 0 {
                info!("Building FOV (0%).");
            } else if i >= frac_size-1 {
                i=0;
                j+=1;
                if j == (1./frac_value) as i32 {
                    info!("Building FOV (100%).");
                    info!("FOV Done!");
                } else {
                    info!("Building FOV ({}%).", j as f32 * (frac_value*100.));
                }
                
            }

            update_cell_fov(
                &mut new_fov_data,
                &precalculated_fov_data,
                &gridmap_main,
                &non_blocking_cells_list,
                vector,
                &query_pipeline,
                &collider_set
            );

            i+=1;

        }

        world_fov.to_be_recalculated = vec![];

        for (key, value) in new_fov_data {

            world_fov.data.insert(key, value);
    
        }

        return;

    }

    
    for _i in 0..MAX_CELLS_PER_TICK {

        // Non blocking load FOV with priorities
        
        let viewpoint_id;
        let mut priority_queue = false;

        if world_fov.to_be_recalculated_priority.len() > 0 {

            viewpoint_id = world_fov.to_be_recalculated_priority[0];
            priority_queue=true;

        } else if world_fov.to_be_recalculated.len() >0 {
            viewpoint_id = world_fov.to_be_recalculated[0];
        } else {
            break;
        }

        update_cell_fov(
            &mut new_fov_data,
            &precalculated_fov_data,
            &gridmap_main,
            &non_blocking_cells_list,
            &viewpoint_id,
            &query_pipeline,
            &collider_set
        );

        if priority_queue {
            let index_option = world_fov.to_be_recalculated.iter().position(|&r| r == viewpoint_id);

            match index_option {
                Some(index) => {
                    world_fov.to_be_recalculated.remove(index);
                },
                None => {},
            }
            
            world_fov.to_be_recalculated_priority.remove(0);

        } else {

            world_fov.to_be_recalculated.remove(0);

        }
        

    }

    for (key, value) in new_fov_data {

        world_fov.data.insert(key, value);

    }


}

fn _update_cell_fov2(
    _new_fov_data : &mut HashMap<Vec2Int, Vec<Vec2Int>>,
    _precalculated_fov_data : &Res<PrecalculatedFOVData>,
    _gridmap_main : &Res<GridmapMain>,
    _non_blocking_cells_list : &Res<NonBlockingCellsList>,
    _viewpoint_cell_id : &Vec2Int,
    query_pipeline: &Res<QueryPipeline>,
    collider_set : &QueryPipelineColliderComponentsSet
) {

    // Dummy function that can be used to demonstrate our custom function is faster than if we were to simply raytrace a few points for each cells.

    let total_cells_in_view_amount = (VIEW_DISTANCE*2) * (VIEW_DISTANCE*2) + 2 * (VIEW_DISTANCE*2);

    for _i in 0..total_cells_in_view_amount {
        
        for _j in 0..3 {

            let target_ray_point = Vec3::new(30.,1.8,30.);
            let origin_ray_point = Vec3::new(0.,1.8,0.);
    
            
    
            let masks = get_bit_masks(ColliderGroup::FOV);
            
            let ray = Ray::new(origin_ray_point.into(), (target_ray_point-origin_ray_point).normalize().into());
            let max_toi = origin_ray_point.distance(target_ray_point);
            let solid = true;
            let groups = InteractionGroups::new(masks.0,masks.1);
            let filter = None;
    
            let mut _is_correct_black_cell = false;
    
            if let Some((_handle, toi)) = query_pipeline.cast_ray(
                collider_set, &ray, max_toi, solid, groups, filter
            ) {
    
                let distance_too_short = max_toi - toi;
    
                if distance_too_short > 1. {
                    _is_correct_black_cell = true;
                }
    
            }

        }

        

    }

}

fn update_cell_fov (
    new_fov_data : &mut HashMap<Vec2Int, Vec<Vec2Int>>,
    precalculated_fov_data : &Res<PrecalculatedFOVData>,
    gridmap_main : &Res<GridmapMain>,
    non_blocking_cells_list : &Res<NonBlockingCellsList>,
    viewpoint_cell_id : &Vec2Int,
    query_pipeline: &Res<QueryPipeline>,
    collider_set : &QueryPipelineColliderComponentsSet
) {


    let fov_cell_start = Vec2Int {
        x: viewpoint_cell_id.x - VIEW_DISTANCE,
        y: viewpoint_cell_id.y - VIEW_DISTANCE,
    };

    let fov_cell_end = Vec2Int {
        x: viewpoint_cell_id.x + VIEW_DISTANCE,
        y: viewpoint_cell_id.y + VIEW_DISTANCE,
    };


    let total_cells_in_view_amount = (VIEW_DISTANCE*2) * (VIEW_DISTANCE*2) + 2 * (VIEW_DISTANCE*2);


    //let mut all_visible_cells = vec![];
    //let mut all_blocking_cells = vec![];
    let mut confirmed_black_cells = vec![];
    let mut forced_black_cells = vec![];
    let mut forced_ray_intersect_cells = vec![];

    let mut square_half_length = 0;
    let mut square_cells_amount = 8;
    let mut square_cells_side_amount = 3;
    let mut current_square_cell = 0;

    let mut iterated_cell_id = Vec2Int {
        x: 0,
        y: 0,
    };

    let mut new_fov : Vec<Vec2Int> = vec![];

    let cell_surface_offsets = [
        Vec3::new(-1.,0.,0.),
        Vec3::new(1.,0.,0.),
        Vec3::new(0.,0.,-1.),
        Vec3::new(0.,0.,1.),
    ];

    //x = unfilled corner . = fov blocker , = this fov blocker
    //This is for unfilled corners.
    let unfilled_corners = [
        //.x
        //x,
        UnfilledCorner {
            empty_offset_left: Vec2Int{ x: -1, y: 0 },
            empty_offset_right: Vec2Int{ x: 0, y: -1 },
            blocker_offset: Vec2Int{ x: -1, y: -1 },
            to_be_shadowed_cell_if_eyes_left: Vec2Int{ x: 0, y: -1 },
            to_be_shadowed_cell_if_eyes_right: Vec2Int{ x: -1, y: 0 },
        },
        //x.
        //,x
        UnfilledCorner {
            empty_offset_left: Vec2Int{ x: 0, y: -1 },
            empty_offset_right: Vec2Int{ x: 1, y: 0 },
            blocker_offset: Vec2Int{ x: 1, y: -1 },
            to_be_shadowed_cell_if_eyes_left: Vec2Int{ x: 1, y: 0 },
            to_be_shadowed_cell_if_eyes_right: Vec2Int{ x: 0, y: -1 },
        },
        //x,
        //.x
        UnfilledCorner {
            empty_offset_left: Vec2Int{ x: -1, y: 0 },
            empty_offset_right: Vec2Int{ x: 0, y: 1 },
            blocker_offset: Vec2Int{ x: -1, y: 1 },
            to_be_shadowed_cell_if_eyes_left: Vec2Int{ x: 0, y: 1 },
            to_be_shadowed_cell_if_eyes_right: Vec2Int{ x: -1, y: 0 },
        },
        //,x
        //x.
        UnfilledCorner {
            empty_offset_left: Vec2Int{ x: 0, y: 1 },
            empty_offset_right: Vec2Int{ x: 1, y: 0 },
            blocker_offset: Vec2Int{ x: 1, y: 1 },
            to_be_shadowed_cell_if_eyes_left: Vec2Int{ x: 1, y: 0 },
            to_be_shadowed_cell_if_eyes_right: Vec2Int{ x: 0, y: 1 },
        },
    ];


    //We iterate from Entity position and then outwards like a square.
	//Then we +1 the halflength and repeat it for a bigger square, until we find our FOV box.
    //We number the cells from the bottom left then go up, right, down and left again. (this may not be accurate because y could be reversed)


    for _i in 0..total_cells_in_view_amount {

        if square_half_length > 0 {

            if current_square_cell < square_cells_side_amount {

                //Left strip, 1st strip

                iterated_cell_id.x = viewpoint_cell_id.x - square_half_length;
                iterated_cell_id.y = viewpoint_cell_id.y - square_half_length + current_square_cell;


            } else if current_square_cell < (square_cells_side_amount * 2 - 1) {

                //Top strip, 2nd strip

                iterated_cell_id.x = viewpoint_cell_id.x - (square_half_length -1) + (current_square_cell - square_cells_side_amount);
                iterated_cell_id.y = viewpoint_cell_id.y + square_half_length;


            } else if current_square_cell < (square_cells_side_amount * 3 - 2) {

                //Right strip, 3rd strip

                iterated_cell_id.x = viewpoint_cell_id.x + square_half_length;
                iterated_cell_id.y = viewpoint_cell_id.y + (square_half_length - 1) - (current_square_cell - square_cells_side_amount - (2*square_half_length));
            

            } else {

                //Bottom strip, 4th strip

                iterated_cell_id.x = viewpoint_cell_id.x + (square_half_length -1) - (current_square_cell - square_cells_side_amount - (4*square_half_length));
                iterated_cell_id.y = viewpoint_cell_id.y - square_half_length;


            }

            current_square_cell+=1;

            if current_square_cell == square_cells_amount {
                current_square_cell = 0;
                square_half_length+=1;
                square_cells_side_amount = (square_half_length * 3) - (square_half_length - 1);
                square_cells_amount = square_half_length * 8;

            }

        } else {
            iterated_cell_id.x = viewpoint_cell_id.x;
            iterated_cell_id.y = viewpoint_cell_id.y;
            square_half_length = 1;
        }

        

        let iterated_relative_cell_id = Vec2Int {
            x: iterated_cell_id.x - viewpoint_cell_id.x,
            y: iterated_cell_id.y - viewpoint_cell_id.y,
        };



        let iterated_cell_data_option = gridmap_main.data.get(&Vec3Int{ x: iterated_cell_id.x, y: 0, z: iterated_cell_id.y });

        let iterated_cell_data;

        match iterated_cell_data_option {
            Some(data) => {
                iterated_cell_data = data;
            },
            None => {
                iterated_cell_data = &CellData {
                    item: -1,
                    orientation: 1,
                };
            },
        }

        let mut iterated_cell_is_visible = false;

        

        if !(iterated_cell_id.x == fov_cell_end.x ||
        iterated_cell_id.x == fov_cell_start.x ||
        iterated_cell_id.y == fov_cell_end.y ||
        iterated_cell_id.y == fov_cell_start.y) {
            

            if !confirmed_black_cells.contains(&iterated_relative_cell_id) {
                iterated_cell_is_visible=true;
            }

            if !non_blocking_cells_list.list.contains(&iterated_cell_data.item) {

                //Check if we just formed an unfilled corner, if so also add the blackcells of that unfilled corner piece to still close the gap.
				//Okay so this is some weird logic and works differently in reality than what this code shows. But with all the odd tweaks it works fine.
				//If you are to edit and make sense of this, may God's grace be with you.

                
                let black_cells_for_blocker_option = precalculated_fov_data.data.get(&iterated_relative_cell_id);

                let mut black_cells_for_blocker;

                match black_cells_for_blocker_option {
                    Some(data) => {
                        black_cells_for_blocker = data.clone();
                    },
                    None => {
                        error!("Accessed out of range data for precalculated_fov_data {:?}.", iterated_relative_cell_id);
                        continue;
                    },
                }

                let mut unfilled_corner_i : i8 = -1;

                for unfilled_corner in unfilled_corners.iter() {

                    unfilled_corner_i+=1;

                    let to_be_checked_cell_data_option = gridmap_main.data.get(&Vec3Int{ 
                        x: iterated_relative_cell_id.x + unfilled_corner.blocker_offset.x,
                        y: 0,
                        z:  iterated_relative_cell_id.y + unfilled_corner.blocker_offset.y
                    });

                    let to_be_checked_cell_data;

                    match to_be_checked_cell_data_option {
                        Some(data) => {
                            to_be_checked_cell_data = data;
                        },
                        None => {
                            to_be_checked_cell_data = &CellData {
                                item: -1,
                                orientation: 1,
                            };
                        },
                    }

                    if non_blocking_cells_list.list.contains(&to_be_checked_cell_data.item) {
                        continue;
                    }


                    let to_be_checked_cell_data_option = gridmap_main.data.get(&Vec3Int{
                        x: iterated_relative_cell_id.x + unfilled_corner.empty_offset_left.x,
                        y: 0,
                        z: iterated_relative_cell_id.y + unfilled_corner.empty_offset_left.y 
                    });

                    let to_be_checked_cell_data;

                    match to_be_checked_cell_data_option {
                        Some(data) => {
                            to_be_checked_cell_data = data;
                        },
                        None => {
                            to_be_checked_cell_data = &CellData {
                                item: -1,
                                orientation: 1,
                            };
                        },
                    }

                    if to_be_checked_cell_data.item != -1 {
                        continue;
                    }

                    let to_be_checked_cell_data_option = gridmap_main.data.get(&Vec3Int{
                        x: iterated_relative_cell_id.x + unfilled_corner.empty_offset_right.x,
                        y: 0,
                        z: iterated_relative_cell_id.y + unfilled_corner.empty_offset_right.y 
                    });

                    let to_be_checked_cell_data;

                    match to_be_checked_cell_data_option {
                        Some(data) => {
                            to_be_checked_cell_data = data;
                        },
                        None => {
                            to_be_checked_cell_data = &CellData {
                                item: -1,
                                orientation: 1,
                            };
                        },
                    }

                    if to_be_checked_cell_data.item != -1 {
                        continue;
                    }

                    let test_value;
                    let mut invert_last_shadow_cell = false;

                    if unfilled_corner_i == 0 {
                        test_value = iterated_relative_cell_id.x < unfilled_corner.empty_offset_left.x || iterated_relative_cell_id.y < unfilled_corner.empty_offset_left.y;
                    } else if unfilled_corner_i == 1 {

                        // Peeking through unfilled corner bug happens here.
                        if iterated_relative_cell_id.x > unfilled_corner.empty_offset_left.x {
                            test_value = iterated_relative_cell_id.x > unfilled_corner.empty_offset_left.x || iterated_relative_cell_id.y < unfilled_corner.empty_offset_left.y;
                        } else {
                            test_value = iterated_relative_cell_id.x < unfilled_corner.empty_offset_left.x || iterated_relative_cell_id.y < unfilled_corner.empty_offset_left.y;
                        }

                        invert_last_shadow_cell = true;

                    } else if unfilled_corner_i == 2 {
                        
                        if iterated_relative_cell_id.x > unfilled_corner.empty_offset_left.x {
                            test_value = iterated_relative_cell_id.x < unfilled_corner.empty_offset_left.x || iterated_relative_cell_id.y < unfilled_corner.empty_offset_left.y;
                        } else {
                            test_value = iterated_relative_cell_id.x > unfilled_corner.empty_offset_left.x || iterated_relative_cell_id.y < unfilled_corner.empty_offset_left.y;
                        }

                    } else {
                        test_value = iterated_relative_cell_id.x < unfilled_corner.empty_offset_left.x || iterated_relative_cell_id.y > unfilled_corner.empty_offset_left.y;
                    }

                    let mut to_be_shadowed_cell_vector;
                    let mut last_shadow_cell_vector;
                    let mut add_to_forced_ray_intersect = false;

                    if test_value {

                        to_be_shadowed_cell_vector = Vec2Int {
                            x: iterated_relative_cell_id.x + unfilled_corner.to_be_shadowed_cell_if_eyes_right.x,
                            y: iterated_relative_cell_id.y + unfilled_corner.to_be_shadowed_cell_if_eyes_right.y,
                        };

                        if invert_last_shadow_cell {

                            last_shadow_cell_vector = Vec2Int {
                                x: iterated_relative_cell_id.x + unfilled_corner.to_be_shadowed_cell_if_eyes_left.x,
                                y: iterated_relative_cell_id.y + unfilled_corner.to_be_shadowed_cell_if_eyes_left.y,
                            };

                            if iterated_relative_cell_id.y + unfilled_corner.empty_offset_left.y >= 0 {

                                to_be_shadowed_cell_vector = Vec2Int {
                                    x: iterated_relative_cell_id.x + unfilled_corner.blocker_offset.x,
                                    y: iterated_relative_cell_id.y + unfilled_corner.blocker_offset.y,
                                };

                                add_to_forced_ray_intersect = true;

                            } else {

                                if last_shadow_cell_vector.x == last_shadow_cell_vector.y {

                                    last_shadow_cell_vector = Vec2Int {
                                        x: (iterated_relative_cell_id.x + unfilled_corner.blocker_offset.x) - 1,
                                        y: iterated_relative_cell_id.y + unfilled_corner.blocker_offset.y,
                                    };

                                    let new_value = Vec2Int {
                                        x: last_shadow_cell_vector.x + viewpoint_cell_id.x,
                                        y: last_shadow_cell_vector.y + viewpoint_cell_id.y
                                    };

                                    if !forced_black_cells.contains(&new_value) {
                                        forced_black_cells.push(new_value);
                                    }

                                    

                                } else {

                                    last_shadow_cell_vector = Vec2Int {
                                        x: iterated_relative_cell_id.x + unfilled_corner.to_be_shadowed_cell_if_eyes_right.x,
                                        y: iterated_relative_cell_id.y + unfilled_corner.to_be_shadowed_cell_if_eyes_right.y,
                                    };

                                }

                            }

                        } else {

                            last_shadow_cell_vector = Vec2Int {
                                x: iterated_relative_cell_id.x + unfilled_corner.to_be_shadowed_cell_if_eyes_right.x,
                                y: iterated_relative_cell_id.y + unfilled_corner.to_be_shadowed_cell_if_eyes_right.y,
                            };

                        }


                    } else {

                        to_be_shadowed_cell_vector = Vec2Int {
                            x: iterated_relative_cell_id.x + unfilled_corner.to_be_shadowed_cell_if_eyes_left.x,
                            y: iterated_relative_cell_id.y + unfilled_corner.to_be_shadowed_cell_if_eyes_left.y,
                        };

                        last_shadow_cell_vector = Vec2Int {
                            x: iterated_relative_cell_id.x + unfilled_corner.to_be_shadowed_cell_if_eyes_left.x,
                            y: iterated_relative_cell_id.y + unfilled_corner.to_be_shadowed_cell_if_eyes_left.y,
                        };

                    }

                    if add_to_forced_ray_intersect {

                        for shadow_cell in precalculated_fov_data.data.get(&to_be_shadowed_cell_vector).unwrap() {
                            if !forced_ray_intersect_cells.contains(shadow_cell) {
                                forced_ray_intersect_cells.push(*shadow_cell);
                            }
                        }

                    }

                    for shadowed_cell in precalculated_fov_data.data.get(&to_be_shadowed_cell_vector).unwrap() {
                        if !black_cells_for_blocker.contains(shadowed_cell) {
                            black_cells_for_blocker.push(*shadowed_cell);
                        }
                    }
                    for shadowed_cell in precalculated_fov_data.data.get(&last_shadow_cell_vector).unwrap() {
                        if !black_cells_for_blocker.contains(shadowed_cell) {
                            black_cells_for_blocker.push(*shadowed_cell);
                        }
                    }
                    
                    break;
                    
                }


                for new_black_cell in black_cells_for_blocker {

                    //Conservatively rayintersect to make sure up close walls dont shadow eachother.
					//If the x or the y is the exact same as the blocking wall whose blackcells we are checking now. Raytrace ONCE (figure out offset based on relative xy from Entity)
					//to find out if this is a case of walls shadowing eachother OR legitimate shadow.

                    let mut should_raycast = false;
                    let mut offset = Vec3::ZERO;

                    let cell_world_position = cell_id_to_world(Vec3Int{
                        x: new_black_cell.x + viewpoint_cell_id.x,
                        y: 0,
                        z: new_black_cell.y + viewpoint_cell_id.y,
                    });

                    if new_black_cell.x == iterated_relative_cell_id.x {

                        should_raycast = true;

                        if new_black_cell.x < 0 {
                            offset = cell_surface_offsets[1];
                        } else {
                            offset = cell_surface_offsets[0];
                        }

                    } else if new_black_cell.y == iterated_relative_cell_id.y || forced_ray_intersect_cells.contains(&new_black_cell){

                        should_raycast = true;

                        if new_black_cell.y < 0 {
                            offset = cell_surface_offsets[3];
                        } else {
                            offset = cell_surface_offsets[2];
                        }

                    }

                    if should_raycast {

                        let mut target_ray_point = cell_world_position + offset;
                        target_ray_point.y = 1.8;
                        let origin_position = cell_id_to_world(Vec3Int{
                            x: viewpoint_cell_id.x,
                            y: 0,
                            z: viewpoint_cell_id.y,
                        });
                        let origin_ray_point = Vec3::new(origin_position.x, 1.8, origin_position.z);

                        

                        let masks = get_bit_masks(ColliderGroup::FOV);
                        
                        let ray = Ray::new(origin_ray_point.into(), (target_ray_point-origin_ray_point).normalize().into());
                        let max_toi = origin_ray_point.distance(target_ray_point);
                        let solid = true;
                        let groups = InteractionGroups::new(masks.0,masks.1);
                        let filter = None;

                        let mut is_correct_black_cell = false;

                        if let Some((_handle, toi)) = query_pipeline.cast_ray(
                            collider_set, &ray, max_toi, solid, groups, filter
                        ) {

                            let distance_too_short = max_toi - toi;

                            if distance_too_short > 1. {
                                is_correct_black_cell = true;
                            }

                        }

                        if is_correct_black_cell {
                            if !confirmed_black_cells.contains(&new_black_cell) {
                                confirmed_black_cells.push(new_black_cell);
                            }

                        } else {

                            if confirmed_black_cells.contains(&new_black_cell) {
                                confirmed_black_cells.remove(confirmed_black_cells.iter().position(|&r| r == new_black_cell).unwrap());
                            }

                        }


                    } else {
                        if !confirmed_black_cells.contains(&new_black_cell) {
                            confirmed_black_cells.push(new_black_cell);
                        }
                    }

                    

                }


            }


        }

        //Fix a few buggy cells here.

        if iterated_cell_is_visible {

            if (iterated_cell_id.y == viewpoint_cell_id.y - 1 ||
                iterated_cell_id.y == viewpoint_cell_id.y + 1) &&
            (iterated_cell_id.x == viewpoint_cell_id.x - 10 ||
            iterated_cell_id.x == viewpoint_cell_id.x + 10 ||
            iterated_cell_id.x == viewpoint_cell_id.x - 8 ||
            iterated_cell_id.x == viewpoint_cell_id.x + 8
            ) {

                let mut offset = Vec3::ZERO;

                if iterated_cell_id.x == viewpoint_cell_id.x - 10 || iterated_cell_id.x == viewpoint_cell_id.x + 10{

                    if iterated_cell_id.y >= viewpoint_cell_id.y {
                        offset = cell_surface_offsets[2];
                    } else {
                        offset = cell_surface_offsets[3];
                    }

                }


                let mut target_ray_point = cell_id_to_world(Vec3Int {
                    x:iterated_cell_id.x,
                    y:0,
                    z: iterated_cell_id.y
                }) + offset;

                target_ray_point.y = 1.8;

                let origin_position = cell_id_to_world(Vec3Int{
                    x: viewpoint_cell_id.x,
                    y: 0,
                    z: viewpoint_cell_id.y,
                });
                let origin_ray_point = Vec3::new(origin_position.x, 1.8, origin_position.z);


                let masks = get_bit_masks(ColliderGroup::FOV);
                let ray = Ray::new(origin_ray_point.into(), (target_ray_point - origin_ray_point).normalize().into());
                let max_toi = origin_ray_point.distance(target_ray_point);
                let solid = true;
                let groups = InteractionGroups::new(masks.0,masks.1);
                let filter = None;

                let mut is_correct_black_cell = false;

                if let Some((_handle, toi)) = query_pipeline.cast_ray(
                    collider_set, &ray, max_toi, solid, groups, filter
                ) {

                    let distance_too_short = max_toi - toi;

                    if distance_too_short > 8.5 {
                        is_correct_black_cell = true;
                    }

                }

                if is_correct_black_cell {

                    iterated_cell_is_visible = false;

                }


            }

        }

        if iterated_cell_is_visible {

            new_fov.push(iterated_cell_id.clone())

        } else {

            let vec_position_result = new_fov.iter().position(|&r| r == iterated_cell_id);

            match vec_position_result{
                Some(vec_position) => {
                    new_fov.remove(vec_position);
                },
                None => {},
            }

            
        }


    }


    new_fov_data.insert(*viewpoint_cell_id, new_fov);



}
