use std::collections::HashMap;
use bevy::prelude::{FromWorld, World};
use serde::{Deserialize};

use crate::space_core::components::inventory_item::HitSoundSurface;

use super::doryen_fov::Vec3Int;


pub struct GridmapMain {
    pub data : HashMap<Vec3Int, CellData>
}

#[derive(Deserialize)]
pub struct CellDataWID {
    pub id: String,
    pub item: i64,
    pub orientation: i64
}

pub struct CellData {
    pub item: i64,
    pub orientation: i64,
    pub health : StructureHealth,
}

pub struct StructureHealth {
    pub brute : f32,
    pub burn : f32,
    pub toxin : f32,
    pub health_flags : HashMap<u32, StructureHealthFlag>,
    pub hit_sound_surface : HitSoundSurface,
}

impl Default for StructureHealth {
    fn default() -> Self {
        Self {
            brute:0.,
            burn:0.,
            toxin:0.,
            health_flags:HashMap::new(),
            hit_sound_surface: HitSoundSurface::Metaloid,
        }
    }
}


#[allow(dead_code)]
pub enum StructureHealthFlag {
    Armored,
}

impl FromWorld for GridmapMain {
    fn from_world(_world: &mut World) -> Self {
        GridmapMain {
           data : HashMap::new(), 
        }
    }
}
