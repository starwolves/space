use bevy_core::{FixedTimesteps, Time};
use bevy_internal::prelude::{warn, Res, ResMut};

use crate::space::{
    core::atmospherics::resources::{AtmosEffect, AtmosphericsResource, CELCIUS_KELVIN_OFFSET},
    ATMOS_DIFFUSION_LABEL,
};

const ATMOS_EFFECT_SPEED: f32 = 0.01;

pub const VACUUM_ATMOSEFFECT: AtmosEffect = AtmosEffect {
    target_temperature: -270.45 + CELCIUS_KELVIN_OFFSET,
    temperature_speed: 250.,
    heater: false,

    target_amount: 0.,
    amount_speed: 250.,
    remover: true,
};

pub fn atmos_effects(
    time: Res<Time>,
    fixed_timesteps: Res<FixedTimesteps>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
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

    for atmospherics in atmospherics_resource.atmospherics.iter_mut() {
        let mut total_amount_additive = 0.;
        let mut total_temperature_additive = 0.;

        for effect in atmospherics.effects.values() {
            if !effect.heater {
                if atmospherics.temperature > effect.target_temperature {
                    total_temperature_additive -=
                        effect.target_temperature * effect.temperature_speed;
                }
            } else {
                if atmospherics.temperature < effect.target_temperature {
                    total_temperature_additive +=
                        effect.target_temperature * effect.temperature_speed;
                }
            }

            if effect.remover {
                if atmospherics.amount > effect.target_amount {
                    total_amount_additive -= effect.target_amount * effect.amount_speed;
                }
            } else {
                if atmospherics.amount < effect.target_amount {
                    total_amount_additive += effect.target_amount * effect.amount_speed;
                }
            }
        }

        atmospherics.amount += total_amount_additive * ATMOS_EFFECT_SPEED;
        atmospherics.temperature += total_temperature_additive * ATMOS_EFFECT_SPEED;

        if atmospherics.amount < 0. {
            atmospherics.amount = 0.;
        }

        if atmospherics.temperature < -270.45 + CELCIUS_KELVIN_OFFSET {
            atmospherics.temperature = -270.45 + CELCIUS_KELVIN_OFFSET;
        }
    }
}
