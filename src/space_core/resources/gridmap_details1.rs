use std::collections::HashMap;

use bevy::prelude::{FromWorld, World};

use super::{doryen_fov::Vec3Int, gridmap_main::CellData};


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
