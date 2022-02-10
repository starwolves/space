use bevy::{prelude::Component};

use super::resources::Vec3Int;

#[derive(Component)]
pub struct Cell {
    pub id : Vec3Int,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            id : Vec3Int{x:0,y:0,z:0},
        }
    }
}

#[derive(Clone, Copy)]
pub struct Atmospherics {
    pub blocked : bool,
    //Kelvin
    pub temperature : f32,
    //Mol
    pub amount : f32,
}

impl Default for Atmospherics {
    fn default() -> Self {
        Self {
            blocked : false,
            temperature : -270.45 + CELCIUS_KELVIN_OFFSET,
            amount: 0.,
        }
    }
}

const CELCIUS_KELVIN_OFFSET : f32 = 273.15;

impl Atmospherics {
    pub fn new_internal() -> Self {
        Self {
            blocked : false,
            temperature : 20. + CELCIUS_KELVIN_OFFSET,
            amount: 84.58,
        }
    }
}
