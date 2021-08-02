
use bevy::prelude::{Transform};

use crate::space_core::components::{entity_data::{EntityData}, entity_updates::EntityUpdates, sensable::Sensable, sfx::Sfx, static_transform::StaticTransform};

pub struct AirLockDeniedSfxBundle;

pub const PLAY_BACK_DURATION : f32 = 1.5 + 1.;

impl AirLockDeniedSfxBundle {

    pub fn new(passed_transform : Transform) -> (
        StaticTransform,
        EntityData,
        Sensable,
        Sfx,
        EntityUpdates
    ) {


        (StaticTransform {
            transform: passed_transform,
        },
        EntityData {
            entity_class : "SFX".to_string(),
            ..Default::default()
        },
        Sensable {
            is_audible: true,
            ..Default::default()
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
            unit_db: 15.,
            unit_size: 1.,
            stream_id: "doorAccessDenied".to_string(),
            play_back_position: 0.,
            play_back_duration: PLAY_BACK_DURATION,
            auto_destroy: true,
            sfx_replay : false
        },
        EntityUpdates::default(),
    )

    }

}
