
use bevy::{prelude::{ Transform}};

use crate::space_core::{generics::{sfx::components::{Sfx, AmbienceSfxTimer}, static_body::components::StaticTransform, entity::components::{EntityData, Sensable, EntityUpdates}}};

pub struct AmbienceSfxBundle;

pub const AMBIENCE_SFX_PLAY_BACK_DURATION : f32 = 424. + 1.;

impl AmbienceSfxBundle {
    
    pub fn new(passed_transform : Transform) -> (
        StaticTransform,
        EntityData,
        Sensable,
        Sfx,
        EntityUpdates,
        AmbienceSfxTimer
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
            always_sensed: true,
            ..Default::default()
        },
        Sfx {
            unit_db: 21.,
            stream_id: "spaceshipAmbientSound".to_string(),
            play_back_position: 0.,
            play_back_duration: AMBIENCE_SFX_PLAY_BACK_DURATION,
            auto_destroy : false,
            ..Default::default()
        },
        EntityUpdates::default(),
        AmbienceSfxTimer::default(),
    )
        

    }

}
