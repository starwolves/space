use bevy_transform::components::Transform;

use crate::space::core::{
    entity::components::{EntityData, EntityUpdates},
    sensable::components::Sensable,
    sfx::components::{get_random_pitch_scale, Sfx},
    static_body::components::StaticTransform,
};

pub struct AirLockOpenSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 4.5 + 1.;

impl AirLockOpenSfxBundle {
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
                unit_db: 13.,
                stream_id: "/content/audio/airLock/doorOpen.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.6),
                ..Default::default()
            },
            EntityUpdates::default(),
        )
    }
}
