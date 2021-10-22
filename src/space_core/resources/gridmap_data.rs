use bevy::prelude::{FromWorld, World};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct GridmapData {
    pub non_fov_blocking_cells_list: Vec<i64>,
    pub non_combat_obstacle_cells_list: Vec<i64>,
    pub non_laser_obstacle_cells_list: Vec<i64>,
    pub placeable_items_cells_list: Vec<i64>,
}

impl FromWorld for GridmapData {
    fn from_world(_world: &mut World) -> Self {
        GridmapData {
            non_fov_blocking_cells_list : vec![],
            non_combat_obstacle_cells_list : vec![],
            non_laser_obstacle_cells_list : vec![],
            placeable_items_cells_list : vec![],
        }
    }
}
