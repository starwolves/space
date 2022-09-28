use std::collections::HashMap;

use api::{
    data::Vec2Int,
    gridmap::{get_atmos_index, FOV_MAP_WIDTH},
};
use bevy::{
    math::Vec3,
    prelude::{warn, Entity, Res, ResMut},
    time::{FixedTimesteps, Time},
};

use super::plugin::ATMOS_DIFFUSION_LABEL;

/// Resource with accumulated rigid body forces of the atmospherics tick per entity, could be building for multiple frames.
#[derive(Default)]
pub(crate) struct RigidBodyForcesAccumulation {
    pub data: HashMap<Entity, Vec<Vec3>>,
}

/// Between 0 and 1.
const TEMPERATURE_DIFFUSIVITY: f32 = 1.;
/// Between 0 and 1.
const AMOUNT_DIFFUSIVITY: f32 = 1.;

/// The higher this is the more CPU intensive and the faster diffusion will take place.
pub const DIFFUSION_STEP: f64 = 28.;

/// Diffuse atmospherics.
pub(crate) fn atmos_diffusion(
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

/// The resource with the  atmospherics states and data of each tile.
pub struct AtmosphericsResource {
    /// Get a tile from this list with [get_atmos_index].
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
    /// Check if cell id is out of atmospherics range.
    pub fn is_id_out_of_range(id: Vec2Int) -> bool {
        let half_width = FOV_MAP_WIDTH as i16 / 2;
        let range = -half_width..half_width;
        let in_range = range.contains(&id.x) && range.contains(&id.y);
        !in_range
    }
}

/// This struct gets repeated [FOV_MAP_WIDTH]*[FOV_MAP_WIDTH] times in [AtmosphericsResource].
#[derive(Clone)]
pub struct Atmospherics {
    /// If blocked by world ie wall.
    pub blocked: bool,
    ///Kelvin
    pub temperature: f32,
    ///Mol
    pub amount: f32,
    /// Add data flags for the tile as unhygienic strings, a little extra identifiers addition for other modules to read to make atmos tiles uniquely identifiable.
    pub flags: Vec<String>,
    /// Add atmospherics effect for this tile, such as atmospherics drainers ie vacuum.
    pub effects: HashMap<EffectType, AtmosEffect>,
    /// If physics should push up on this tile.
    pub forces_push_up: bool,
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

/// 273.15
pub const CELCIUS_KELVIN_OFFSET: f32 = 273.15;
/// 84.58
pub const DEFAULT_INTERNAL_AMOUNT: f32 = 84.58;

impl Atmospherics {
    /// Create default atmospherics for internal tiles.
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
    /// Get pressure in kpa.
    pub fn get_pressure(&self) -> f32 {
        (((self.amount * 0.08206 * self.temperature) / 2000.) * 101325.) / 1000.
    }
}

/// The default atmospherics effect for vacuum.
pub const VACUUM_ATMOSEFFECT: AtmosEffect = AtmosEffect {
    target_temperature: -270.45 + CELCIUS_KELVIN_OFFSET,
    temperature_speed: 500.,
    heater: false,

    target_amount: 0.,
    amount_speed: 500.,
    remover: true,
};

/// An atmospheric effect type.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EffectType {
    Floorless,
    Entity(Entity),
}

/// An atmospheric effect.
#[derive(Clone, Debug)]
pub struct AtmosEffect {
    /// The temperature this effect tries to bring the tile to.
    pub target_temperature: f32,
    /// The intensity at which this effect will change the temperature.
    pub temperature_speed: f32,
    /// Whether the effect is heating or cooling.
    pub heater: bool,
    /// The amount in mol this effect tries to bring the tile to.
    pub target_amount: f32,
    /// The intensity at which this effect will change the amount.
    pub amount_speed: f32,
    /// Whether this effect adds or removes matter.
    pub remover: bool,
}
