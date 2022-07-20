use std::collections::HashMap;

use bevy::prelude::Entity;

// This struct gets repeated FOV_MAP_WIDTH*FOV_MAP_WIDTH (250k) times in our atmospherics dictionary.
#[derive(Clone)]
pub struct Atmospherics {
    pub blocked: bool,
    //Kelvin
    pub temperature: f32,
    //Mol
    pub amount: f32,
    pub flags: Vec<String>,
    pub effects: HashMap<EffectType, AtmosEffect>,
    pub forces_push_up: bool,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EffectType {
    Floorless,
    Entity(Entity),
}

#[derive(Clone, Debug)]
pub struct AtmosEffect {
    pub target_temperature: f32,
    pub temperature_speed: f32,
    pub heater: bool,

    pub target_amount: f32,
    pub amount_speed: f32,
    pub remover: bool,
}

impl Default for Atmospherics {
    fn default() -> Self {
        let mut effects = HashMap::new();
        effects.insert(EffectType::Floorless, VACUUM_ATMOSEFFECT);
        Self {
            blocked: false,
            temperature: -270.45 + CELCIUS_KELVIN_OFFSET,
            amount: 0.,
            effects: effects,
            flags: vec![],
            forces_push_up: false,
        }
    }
}

pub const CELCIUS_KELVIN_OFFSET: f32 = 273.15;
pub const DEFAULT_INTERNAL_AMOUNT: f32 = 84.58;

impl Atmospherics {
    pub fn new_internal(blocked: bool, forces_push_up: bool) -> Self {
        Self {
            blocked,
            temperature: 20. + CELCIUS_KELVIN_OFFSET,
            amount: DEFAULT_INTERNAL_AMOUNT,
            effects: HashMap::new(),
            flags: vec![],
            forces_push_up,
        }
    }
    pub fn get_pressure(&self) -> f32 {
        // Return kpa
        (((self.amount * 0.08206 * self.temperature) / 2000.) * 101325.) / 1000.
    }
}

pub const VACUUM_ATMOSEFFECT: AtmosEffect = AtmosEffect {
    target_temperature: -270.45 + CELCIUS_KELVIN_OFFSET,
    temperature_speed: 500.,
    heater: false,

    target_amount: 0.,
    amount_speed: 500.,
    remover: true,
};
