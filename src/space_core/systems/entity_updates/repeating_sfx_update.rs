use bevy::prelude::{Changed, Query};

use crate::space_core::{components::{entity_updates::EntityUpdates, repeating_sfx::RepeatingSfx}, functions::entity_updates::get_entity_update_difference::get_entity_update_difference, resources::network_messages::EntityUpdateData};

pub fn repeating_sfx_update(
    mut updated_sfx: Query<(&RepeatingSfx, &mut EntityUpdates), Changed<RepeatingSfx>>,
) {

    for (sfx_component, mut entity_updates_component) in updated_sfx.iter_mut() {

        let old_entity_updates = entity_updates_component.updates.clone();

        let entity_updates = entity_updates_component.updates
        .get_mut(&".".to_string()).unwrap();

        entity_updates.insert(
            "area_mask".to_string(),
            EntityUpdateData::UInt8(sfx_component.area_mask)
        );
        entity_updates.insert(
            "attenuation_filter_cutoff_hz".to_string(),
            EntityUpdateData::Float(sfx_component.attenuation_filter_cutoff_hz)
        );
        entity_updates.insert(
            "attenuation_filter_db".to_string(),
            EntityUpdateData::Float(sfx_component.attenuation_filter_db)
        );
        entity_updates.insert(
            "repeat_time".to_string(),
            EntityUpdateData::Float(sfx_component.repeat_time)
        );
        entity_updates.insert(
            "attenuation_model".to_string(),
            EntityUpdateData::UInt8(sfx_component.attenuation_model)
        );
        entity_updates.insert(
            "auto_play".to_string(),
            EntityUpdateData::Bool(sfx_component.auto_play)
        );
        entity_updates.insert(
            "bus".to_string(),
            EntityUpdateData::String(sfx_component.bus.clone())
        );
        entity_updates.insert(
            "doppler_tracking".to_string(),
            EntityUpdateData::UInt8(sfx_component.doppler_tracking)
        );
        entity_updates.insert(
            "emission_angle_degrees".to_string(),
            EntityUpdateData::Float(sfx_component.emission_angle_degrees)
        );
        entity_updates.insert(
            "emission_angle_enabled".to_string(),
            EntityUpdateData::Bool(sfx_component.emission_angle_enabled)
        );
        entity_updates.insert(
            "emission_angle_filter_attenuation_db".to_string(),
            EntityUpdateData::Float(sfx_component.emission_angle_filter_attenuation_db)
        );
        entity_updates.insert(
            "max_db".to_string(),
            EntityUpdateData::Float(sfx_component.max_db)
        );
        entity_updates.insert(
            "max_distance".to_string(),
            EntityUpdateData::Float(sfx_component.max_distance)
        );
        entity_updates.insert(
            "out_of_range_mode".to_string(),
            EntityUpdateData::UInt8(sfx_component.out_of_range_mode)
        );
        entity_updates.insert(
            "pitch_scale".to_string(),
            EntityUpdateData::Float(sfx_component.pitch_scale)
        );
        entity_updates.insert(
            "playing".to_string(),
            EntityUpdateData::Bool(sfx_component.playing)
        );
        entity_updates.insert(
            "stream_paused".to_string(),
            EntityUpdateData::Bool(sfx_component.stream_paused)
        );
        entity_updates.insert(
            "unit_db".to_string(),
            EntityUpdateData::Float(sfx_component.unit_db)
        );
        entity_updates.insert(
            "unit_size".to_string(),
            EntityUpdateData::Float(sfx_component.unit_size)
        );
        entity_updates.insert(
            "stream_id".to_string(),
            EntityUpdateData::String(sfx_component.stream_id.clone())
        );

        let difference_updates = get_entity_update_difference(
            old_entity_updates,
            &entity_updates_component.updates
        );

        entity_updates_component.updates_difference = difference_updates;

    }

}
