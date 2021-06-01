use std::collections::HashMap;

use bevy::{core::Timer, prelude::{ Transform}};

use crate::space_core::components::{ambience_sfx_timer::AmbienceSfxTimer, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, sensable::Sensable, sfx::Sfx, static_transform::StaticTransform};

pub struct AmbienceSfxBundle;

pub const PLAY_BACK_DURATION : f32 = 424. + 1.;
// pub const PLAY_BACK_DURATION : f32 = 12. + 1.;

impl AmbienceSfxBundle {
    
    pub fn new(passed_transform : Transform) -> (
        StaticTransform,
        EntityData,
        Sensable,
        Sfx,
        EntityUpdates,
        AmbienceSfxTimer
    ) {

        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());

        

        (StaticTransform {
            transform: passed_transform,
        },
        EntityData {
            entity_class : "SFX".to_string(),
            entity_type: "".to_string(),
            entity_group : EntityGroup::None
        },
        Sensable {
            is_light : false,
            is_audible: true,
            sensed_by : vec![],
            sensed_by_cached : vec![],
            always_sensed : true
        },
        Sfx {
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
            pitch_scale: 1.,
            playing: false,
            stream_paused: false,
            unit_db: 11.,
            unit_size: 1.,
            stream_id: "spaceshipAmbientSound".to_string(),
            play_back_position: 0.,
            play_back_duration: PLAY_BACK_DURATION,
            auto_destroy : true,
            sfx_replay : false
        },
        EntityUpdates {
            updates: entity_updates_map,
            changed_parameters: vec![]
        },
        AmbienceSfxTimer {
            timer : Timer::from_seconds(PLAY_BACK_DURATION, false)
        })
        

    }

}
