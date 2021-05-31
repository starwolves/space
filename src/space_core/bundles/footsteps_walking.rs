use std::collections::HashMap;

use bevy::prelude::{Transform};

use crate::space_core::components::{cached_broadcast_transform::CachedBroadcastTransform, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, footsteps_walking::FootstepsWalking, repeating_sfx::RepeatingSfx, sensable::Sensable, static_transform::StaticTransform, update_transform::UpdateTransform};

pub struct FootstepsWalkingSfxBundle;

impl FootstepsWalkingSfxBundle {

    pub fn new(passed_transform : Transform) -> (
        StaticTransform,
        EntityData,
        Sensable,
        RepeatingSfx,
        EntityUpdates,
        FootstepsWalking,
        UpdateTransform,
        CachedBroadcastTransform
    ) {

        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());

        (StaticTransform {
            transform: passed_transform,
        },
        EntityData {
            entity_class : "RepeatingSFX".to_string(),
            entity_type: "".to_string(),
            entity_group : EntityGroup::None
        },
        Sensable {
            is_light : false,
            is_audible: true,
            sensed_by : vec![],
            sensed_by_cached : vec![]
        },
        RepeatingSfx {
            area_mask: 0,
            attenuation_filter_cutoff_hz: 5000.,
            attenuation_filter_db: -24.,
            attenuation_model: 0,
            auto_play: true,
            bus: "Master".to_string(),
            doppler_tracking: 0,
            emission_angle_degrees: 45.,
            emission_angle_enabled: false,
            emission_angle_filter_attenuation_db: -12.,
            max_db: 3.,
            max_distance: 0.,
            out_of_range_mode: 0,
            pitch_scale: 1.6,
            playing: false,
            stream_paused: false,
            unit_db: 6.5,
            unit_size: 1.,
            stream_id: "concrete_walking_footsteps".to_string(),
            auto_destroy : true,
            repeat_time: 0.5,
        },
        EntityUpdates {
            updates: entity_updates_map
        },
        FootstepsWalking,
        UpdateTransform,
        CachedBroadcastTransform::new())

    }

}
