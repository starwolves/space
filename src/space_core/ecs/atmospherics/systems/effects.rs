use bevy::prelude::ResMut;

use crate::space_core::ecs::atmospherics::resources::AtmosphericsResource;

pub fn atmos_effects(
    mut atmospherics_resource : ResMut<AtmosphericsResource>,
) {

    for atmospherics in atmospherics_resource.atmospherics.iter_mut() {

        let mut total_amount_additive = 0.;
        let mut total_temperature_additive = 0.;

        for effect in atmospherics.effects.values() {

            total_amount_additive += effect.amount_additive;
            total_temperature_additive += effect.temperature_additive;

        }

        atmospherics.amount += total_amount_additive * 0.01;
        atmospherics.temperature += total_temperature_additive * 0.01;


    }

}
