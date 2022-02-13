use std::collections::HashMap;

use bevy::prelude::{FromWorld, World, Entity};

use crate::space_core::ecs::gridmap::{resources::FOV_MAP_WIDTH, systems::remove_cell::VACUUM_ATMOSEFFECT};

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


// This struct gets repeated FOV_MAP_WIDTH*FOV_MAP_WIDTH (250k) times in our atmospherics dictionary.
#[derive(Clone)]
pub struct Atmospherics {
    pub blocked : bool,
    //Kelvin
    pub temperature : f32,
    //Mol
    pub amount : f32,
    pub flags : Vec<String>,
    pub effects : HashMap<EffectType, AtmosEffect>,
}

#[derive(Clone, PartialEq,Eq, Hash)]
pub enum EffectType {
    Floorless,
    Entity(Entity),
}

#[derive(Clone)]
pub struct AtmosEffect {
    pub target_temperature : f32,
    pub temperature_speed : f32,
    pub heater : bool,

    pub target_amount : f32,
    pub amount_speed : f32,
    pub remover : bool,
}

impl Default for Atmospherics {
    fn default() -> Self {
        let mut effects = HashMap::new();
        effects.insert(EffectType::Floorless, VACUUM_ATMOSEFFECT);
        Self {
            blocked : false,
            temperature : -270.45 + CELCIUS_KELVIN_OFFSET,
            amount: 0.,
            effects : effects,
            flags: vec![],
        }
    }
}

pub const CELCIUS_KELVIN_OFFSET : f32 = 273.15;

impl Atmospherics {
    pub fn new_internal() -> Self {
        Self {
            blocked : false,
            temperature : 20. + CELCIUS_KELVIN_OFFSET,
            amount: 84.58,
            effects : HashMap::new(),
            flags: vec![],
        }
    }
    pub fn get_pressure(&self) -> f32 {
        // Return kpa
        (((self.amount*0.08206*self.temperature)/2000.)*101325.)/1000.
    }
}

#[derive(Default)]
pub struct MapHolders {
    pub holders : HashMap<Entity, MapHolderData>,
}

pub struct MapHolderData {
    pub batch_i : usize,
}
