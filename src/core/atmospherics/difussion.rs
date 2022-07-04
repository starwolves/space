use std::collections::HashMap;

use bevy::{
    core::{FixedTimesteps, Time},
    math::Vec3,
    prelude::{warn, Entity, Res, ResMut},
};

use crate::core::{
    gridmap::gridmap::{Vec2Int, FOV_MAP_WIDTH},
    map::map_overlay::OverlayTile,
};

use super::{effects::VACUUM_ATMOSEFFECT, plugin::ATMOS_DIFFUSION_LABEL};

pub fn get_atmos_index(id: Vec2Int) -> usize {
    let idx: u32 = (id.x + (FOV_MAP_WIDTH / 2) as i16) as u32;
    let idy: u32 = (id.y + (FOV_MAP_WIDTH / 2) as i16) as u32;

    (idx + (idy * FOV_MAP_WIDTH as u32)) as usize
}

pub fn get_atmos_id(i: usize) -> Vec2Int {
    let y = (i as f32 / FOV_MAP_WIDTH as f32).floor() as usize;
    let x = i - (y * FOV_MAP_WIDTH);

    Vec2Int {
        x: x as i16 - (FOV_MAP_WIDTH as i16 / 2),
        y: y as i16 - (FOV_MAP_WIDTH as i16 / 2),
    }
}

pub struct AtmosphericsResource {
    pub atmospherics: Vec<Atmospherics>,
}

impl Default for AtmosphericsResource {
    fn default() -> Self {
        AtmosphericsResource {
            atmospherics: vec![Atmospherics::default(); FOV_MAP_WIDTH * FOV_MAP_WIDTH],
        }
    }
}

impl AtmosphericsResource {
    pub fn is_id_out_of_range(id: Vec2Int) -> bool {
        let half_width = FOV_MAP_WIDTH as i16 / 2;
        let range = -half_width..half_width;
        let in_range = range.contains(&id.x) && range.contains(&id.y);
        !in_range
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

#[derive(Clone, Default)]
pub struct AtmosphericsCache {
    pub tile_color: Option<OverlayTile>,
}

#[derive(Default)]
pub struct RigidBodyForcesAccumulation {
    pub data: HashMap<Entity, Vec<Vec3>>,
}

// Between 0 and 1
const TEMPERATURE_DIFFUSIVITY: f32 = 1.;
const AMOUNT_DIFFUSIVITY: f32 = 1.;

// The higher this is the more CPU intensive and the faster diffusion will take place.
pub const DIFFUSION_STEP: f64 = 28.;

pub fn atmos_diffusion(
    time: Res<Time>,
    fixed_timesteps: Res<FixedTimesteps>,
    mut atmospherics: ResMut<AtmosphericsResource>,
) {
    let current_time_stamp = time.time_since_startup().as_millis();

    let overstep_percentage = fixed_timesteps
        .get(ATMOS_DIFFUSION_LABEL)
        .unwrap()
        .overstep_percentage();
    if overstep_percentage > 5. {
        if current_time_stamp > 60000 {
            warn!("overstep_percentage: {}", overstep_percentage);
        }
    }

    // Currently we just calculate atmos of all 250k cells.
    // In the future optimize it so it keeps track of enclosed spaces,
    // and when theres an unbalance in an unclosed space diffuse only inside of it until a new balance is reached.
    // This means we can simulate atmospherics of at least 250k active cells at once during a game session, it could probably hanle more too.

    let default_x = FOV_MAP_WIDTH as i16 / 2;

    let mut current_cell_id = Vec2Int {
        x: -default_x - 1,
        y: -default_x,
    };

    let vacuum_atmos = Atmospherics::default();

    // Takes about 1ms.
    for _i in 0..FOV_MAP_WIDTH * FOV_MAP_WIDTH {
        current_cell_id.x += 1;

        if current_cell_id.x > default_x {
            current_cell_id.x = -default_x;
            current_cell_id.y += 1;
        }

        let current_cell_atmos = atmospherics
            .atmospherics
            .get(get_atmos_index(current_cell_id))
            .unwrap();

        if current_cell_atmos.blocked {
            continue;
        }

        let mut total_temperature = 0.;
        let mut total_amount = 0.;

        let mut non_blocking_adjacents: u8 = 0;

        for j in 0..4 {
            let mut adjacent_cell_id = current_cell_id.clone();

            if j == 0 {
                adjacent_cell_id.x += 1
            } else if j == 1 {
                adjacent_cell_id.x -= 1
            } else if j == 2 {
                adjacent_cell_id.y += 1
            } else {
                adjacent_cell_id.y -= 1
            }

            let out_of_range;

            if AtmosphericsResource::is_id_out_of_range(adjacent_cell_id) {
                out_of_range = true;
            } else {
                match atmospherics
                    .atmospherics
                    .get(get_atmos_index(adjacent_cell_id))
                {
                    Some(a) => {
                        if !a.blocked {
                            non_blocking_adjacents += 1;
                            total_temperature += a.temperature;
                            total_amount += a.amount;
                        }
                        out_of_range = false;
                    }
                    None => {
                        out_of_range = true;
                    }
                }
            }

            if out_of_range {
                // Tile is outside of map range, permanent vacuum.
                total_temperature += vacuum_atmos.temperature;
                total_amount += vacuum_atmos.amount;
            }
        }

        if non_blocking_adjacents == 0 {
            continue;
        }

        //let new_temperature = total_temperature / non_blocking_adjacents as f32;
        //let new_amount = total_amount / non_blocking_adjacents as f32;

        let new_temperature = (current_cell_atmos.temperature
            + TEMPERATURE_DIFFUSIVITY * (total_temperature / non_blocking_adjacents as f32))
            / (1. + TEMPERATURE_DIFFUSIVITY);
        let new_amount = (current_cell_atmos.amount
            + AMOUNT_DIFFUSIVITY * (total_amount / non_blocking_adjacents as f32))
            / (1. + AMOUNT_DIFFUSIVITY);

        let current_cell_atmos = atmospherics
            .atmospherics
            .get_mut(get_atmos_index(current_cell_id))
            .unwrap();

        current_cell_atmos.temperature = new_temperature;
        current_cell_atmos.amount = new_amount;
    }
}
