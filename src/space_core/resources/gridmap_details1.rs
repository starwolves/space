use std::collections::HashMap;

use bevy::prelude::{FromWorld, World};

use super::{gridmap_main::CellData, precalculated_fov_data::Vec3Int};

pub struct GridmapDetails1 {
    pub data : HashMap<Vec3Int, CellData>
}


impl FromWorld for GridmapDetails1 {
    fn from_world(_world: &mut World) -> Self {
        GridmapDetails1 {
           data : HashMap::new(), 
        }
    }
}
