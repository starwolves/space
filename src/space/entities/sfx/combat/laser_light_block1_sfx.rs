use bevy_transform::components::Transform;

use crate::space::core::{
    entity::components::{EntityData, EntityUpdates},
    sensable::components::Sensable,
    sfx::components::{get_random_pitch_scale, Sfx},
    static_body::components::StaticTransform,
};

pub struct LaserLightBlock1Bundle;

pub const LASER_LIGHT_BLOCK1_PLAY_BACK_DURATION: f32 = 1.4 + 1.;

impl LaserLightBlock1Bundle {
    pub fn new(
        passed_transform: Transform,
    ) -> (StaticTransform, EntityData, Sensable, Sfx, EntityUpdates) {
        (
            StaticTransform {
                transform: passed_transform,
            },
            EntityData {
                entity_class: "SFX".to_string(),
                ..Default::default()
            },
            Sensable {
                is_audible: true,
                ..Default::default()
            },
            Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/laser_light_block1.sample".to_string(),
                play_back_duration: LASER_LIGHT_BLOCK1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },
            EntityUpdates::default(),
        )
    }
}
