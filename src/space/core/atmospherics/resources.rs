use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    prelude::{FromWorld, World},
};
use bevy_math::Vec3;

use crate::space::core::{
    gridmap::resources::{Vec2Int, FOV_MAP_WIDTH},
    map::functions::OverlayTile,
};

use super::systems::effects::VACUUM_ATMOSEFFECT;

pub struct AtmosphericsResource {
    pub atmospherics: Vec<Atmospherics>,
}

impl FromWorld for AtmosphericsResource {
    fn from_world(_world: &mut World) -> Self {
        AtmosphericsResource {
            atmospherics: vec![Atmospherics::default(); FOV_MAP_WIDTH * FOV_MAP_WIDTH],
        }
    }
}

impl AtmosphericsResource {
    pub fn is_id_out_of_range(id: Vec2Int) -> bool {
        if id.x < -(FOV_MAP_WIDTH as i16 / 2) {
            true
        } else if id.x > FOV_MAP_WIDTH as i16 / 2 {
            true
        } else if id.y < -(FOV_MAP_WIDTH as i16 / 2) {
            true
        } else if id.y > FOV_MAP_WIDTH as i16 / 2 {
            true
        } else {
            false
        }
    }
}

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

#[derive(Default)]
pub struct MapHolders {
    pub holders: HashMap<Entity, MapHolderData>,
}

pub struct MapHolderData {
    pub batch_i: usize,
    pub cache: Vec<AtmosphericsCache>,
    pub prev_camera_cell_id: Vec2Int,
    pub prev_camera_view_range: usize,
    pub reset_cache: bool,
    pub hovering_data: String,
}

#[derive(Clone)]
pub struct AtmosphericsCache {
    pub tile_color: Option<OverlayTile>,
}

impl Default for MapHolderData {
    fn default() -> Self {
        Self {
            batch_i: 0,
            cache: vec![AtmosphericsCache::default(); FOV_MAP_WIDTH * FOV_MAP_WIDTH],
            prev_camera_cell_id: Vec2Int::default(),
            prev_camera_view_range: 20,
            reset_cache: true,
            hovering_data: "".to_string(),
        }
    }
}

impl Default for AtmosphericsCache {
    fn default() -> Self {
        Self { tile_color: None }
    }
}

pub struct RigidBodyForcesAccumulation {
    pub data: HashMap<Entity, Vec<Vec3>>,
}

impl Default for RigidBodyForcesAccumulation {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
