use bevy::prelude::{FromWorld, World};

use crate::space_core::ecs::gridmap::{resources::FOV_MAP_WIDTH};

pub struct AtmosphericsResource {
    pub atmospherics : Vec<Atmospherics>,
}

impl FromWorld for AtmosphericsResource {
    fn from_world(_world: &mut World) -> Self {
        AtmosphericsResource {
            atmospherics: vec![Atmospherics::default(); FOV_MAP_WIDTH*FOV_MAP_WIDTH],
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
