use bevy::prelude::Transform;

use crate::space_core::{generics::{sfx::components::{Sfx, get_random_pitch_scale}, static_body::components::StaticTransform, entity::components::{EntityData, Sensable, EntityUpdates}}};

pub struct LaserLightShot3Bundle;

pub const LASER_LIGHT_SHOT3_PLAY_BACK_DURATION : f32 = 1.8 + 0.8;

impl LaserLightShot3Bundle {
    
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
            unit_db: 15.,
            unit_size: 1.,
            stream_id: "laser_light_shot3".to_string(),
            play_back_duration: LASER_LIGHT_SHOT3_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(3.),
            ..Default::default()
        },
        EntityUpdates::default(),)

    }

}
