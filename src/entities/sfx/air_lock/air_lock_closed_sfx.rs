use bevy_transform::components::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    sensable::components::Sensable,
    sfx::components::{get_random_pitch_scale, Sfx},
};

pub struct AirLockClosedSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1.5 + 1.;

impl AirLockClosedSfxBundle {
    pub fn new(
        passed_transform: Transform,
    ) -> (Transform, EntityData, Sensable, Sfx, EntityUpdates) {
        (
            passed_transform,
            EntityData {
                entity_class: "SFX".to_string(),
                ..Default::default()
            },
            Sensable {
                is_audible: true,
                ..Default::default()
            },
            Sfx {
                unit_db: 19.,
                unit_size: 1.,
                stream_id: "/content/audio/airLock/doorCloseCompression.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },
            EntityUpdates::default(),
        )
    }
}
