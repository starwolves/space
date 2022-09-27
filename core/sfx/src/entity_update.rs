use api::entity_updates::{
    entity_update_changed_detection, get_entity_update_difference, EntityUpdateData, EntityUpdates,
};
use bevy::prelude::{Changed, Entity, Query};

use super::builder::{RepeatingSfx, Sfx};

/// Send repeating sfx entity updates for Godot clients.
pub(crate) fn repeating_sfx_update(
    mut updated_sfx: Query<(&RepeatingSfx, &mut EntityUpdates), Changed<RepeatingSfx>>,
) {
    for (sfx_component, mut entity_updates_component) in updated_sfx.iter_mut() {
        let old_entity_updates = entity_updates_component.updates.clone();

        let entity_updates = entity_updates_component
            .updates
            .get_mut(&".".to_string())
            .unwrap();

        entity_updates.insert(
            "area_mask".to_string(),
            EntityUpdateData::UInt8(sfx_component.area_mask),
        );
        entity_updates.insert(
            "attenuation_filter_cutoff_hz".to_string(),
            EntityUpdateData::Float(sfx_component.attenuation_filter_cutoff_hz),
        );
        entity_updates.insert(
            "attenuation_filter_db".to_string(),
            EntityUpdateData::Float(sfx_component.attenuation_filter_db),
        );
        entity_updates.insert(
            "repeat_time".to_string(),
            EntityUpdateData::Float(sfx_component.repeat_time),
        );
        entity_updates.insert(
            "attenuation_model".to_string(),
            EntityUpdateData::UInt8(sfx_component.attenuation_model),
        );
        entity_updates.insert(
            "auto_play".to_string(),
            EntityUpdateData::Bool(sfx_component.auto_play),
        );
        entity_updates.insert(
            "bus".to_string(),
            EntityUpdateData::String(sfx_component.bus.clone()),
        );
        entity_updates.insert(
            "doppler_tracking".to_string(),
            EntityUpdateData::UInt8(sfx_component.doppler_tracking),
        );
        entity_updates.insert(
            "emission_angle_degrees".to_string(),
            EntityUpdateData::Float(sfx_component.emission_angle_degrees),
        );
        entity_updates.insert(
            "emission_angle_enabled".to_string(),
            EntityUpdateData::Bool(sfx_component.emission_angle_enabled),
        );
        entity_updates.insert(
            "emission_angle_filter_attenuation_db".to_string(),
            EntityUpdateData::Float(sfx_component.emission_angle_filter_attenuation_db),
        );
        entity_updates.insert(
            "max_db".to_string(),
            EntityUpdateData::Float(sfx_component.max_db),
        );
        entity_updates.insert(
            "max_distance".to_string(),
            EntityUpdateData::Float(sfx_component.max_distance),
        );
        entity_updates.insert(
            "out_of_range_mode".to_string(),
            EntityUpdateData::UInt8(sfx_component.out_of_range_mode),
        );
        entity_updates.insert(
            "pitch_scale".to_string(),
            EntityUpdateData::Float(sfx_component.pitch_scale),
        );
        entity_updates.insert(
            "playing".to_string(),
            EntityUpdateData::Bool(sfx_component.playing),
        );
        entity_updates.insert(
            "stream_paused".to_string(),
            EntityUpdateData::Bool(sfx_component.stream_paused),
        );
        entity_updates.insert(
            "unit_db".to_string(),
            EntityUpdateData::Float(sfx_component.unit_db),
        );
        entity_updates.insert(
            "unit_size".to_string(),
            EntityUpdateData::Float(sfx_component.unit_size),
        );
        entity_updates.insert(
            "stream_id".to_string(),
            EntityUpdateData::String(sfx_component.stream_id.clone()),
        );

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);

        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}

/// Send sfx entity updates to Godot clients.
pub(crate) fn sfx_update(mut updated_sfx: Query<(&mut Sfx, &mut EntityUpdates), Changed<Sfx>>) {
    for (mut sfx_component, mut entity_updates_component) in updated_sfx.iter_mut() {
        let old_entity_updates = entity_updates_component.updates.clone();

        entity_updates_component.changed_parameters.clear();

        let entity_updates = entity_updates_component
            .updates
            .get_mut(&".".to_string())
            .unwrap();

        let mut changed_parameters = vec![];

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::UInt8(sfx_component.area_mask),
            "area_mask".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.attenuation_filter_cutoff_hz),
            "attenuation_filter_cutoff_hz".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.attenuation_filter_db),
            "attenuation_filter_db".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::UInt8(sfx_component.attenuation_model),
            "attenuation_model".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Bool(sfx_component.auto_play),
            "auto_play".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::String(sfx_component.bus.clone()),
            "bus".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::UInt8(sfx_component.doppler_tracking),
            "doppler_tracking".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.emission_angle_degrees),
            "emission_angle_degrees".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Bool(sfx_component.emission_angle_enabled),
            "emission_angle_enabled".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.emission_angle_filter_attenuation_db),
            "emission_angle_filter_attenuation_db".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.max_db),
            "max_db".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.max_distance),
            "max_distance".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::UInt8(sfx_component.out_of_range_mode),
            "out_of_range_mode".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.pitch_scale),
            "pitch_scale".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Bool(sfx_component.playing),
            "playing".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Bool(sfx_component.stream_paused),
            "stream_paused".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.unit_db),
            "unit_db".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.unit_size),
            "unit_size".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::String(sfx_component.stream_id.clone()),
            "stream_id".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Bool(sfx_component.auto_destroy),
            "auto_destroy".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Bool(sfx_component.sfx_replay.clone()),
            "sfx_replay".to_string(),
        );

        entity_update_changed_detection(
            &mut changed_parameters,
            entity_updates,
            EntityUpdateData::Float(sfx_component.play_back_position),
            "play_back_position".to_string(),
        );

        if sfx_component.sfx_replay == true {
            sfx_component.sfx_replay = false;
        }

        entity_updates_component.changed_parameters = changed_parameters;

        let difference_updates =
            get_entity_update_difference(old_entity_updates, &entity_updates_component.updates);
        entity_updates_component
            .updates_difference
            .push(difference_updates);
    }
}

#[derive(Default)]
pub struct SfxAutoDestroyTimers {
    pub timers: Vec<(Entity, u8)>,
}
