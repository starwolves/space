use bevy::prelude::ResMut;

use crate::space_core::ecs::atmospherics::resources::{AtmosphericsResource, CELCIUS_KELVIN_OFFSET};

const ATMOS_EFFECT_SPEED : f32 = 0.01;

pub fn atmos_effects(
    mut atmospherics_resource : ResMut<AtmosphericsResource>,
) {

    for atmospherics in atmospherics_resource.atmospherics.iter_mut() {

        let mut total_amount_additive = 0.;
        let mut total_temperature_additive = 0.;

        for effect in atmospherics.effects.values() {

            if !effect.heater {
                if atmospherics.temperature > effect.target_temperature {
                    total_temperature_additive -= effect.target_temperature * effect.temperature_speed;
                }
            } else {
                if atmospherics.temperature < effect.target_temperature {
                    total_temperature_additive += effect.target_temperature * effect.temperature_speed;
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
