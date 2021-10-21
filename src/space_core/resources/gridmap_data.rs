use bevy::prelude::{FromWorld, World};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct GridmapData {
    pub non_blocking_cells_list: Vec<i64>,
    pub non_obstacle_cells_list: Vec<i64>,
    pub non_laser_obstacle_cells_list: Vec<i64>,
}

impl FromWorld for GridmapData {
    fn from_world(_world: &mut World) -> Self {
        GridmapData {
            non_blocking_cells_list : vec![],
            non_obstacle_cells_list : vec![],
            non_laser_obstacle_cells_list : vec![],
        }
    }
}
