use bevy_transform::components::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    sensable::components::Sensable,
    sfx::components::{AmbienceSfxTimer, Sfx},
};

pub struct AmbienceSfxBundle;

pub const AMBIENCE_SFX_PLAY_BACK_DURATION: f32 = 424. + 1.;

impl AmbienceSfxBundle {
    pub fn new(
        passed_transform: Transform,
    ) -> (
        Transform,
        EntityData,
        Sensable,
        Sfx,
        EntityUpdates,
        AmbienceSfxTimer,
    ) {
        (
            passed_transform,
            EntityData {
                entity_class: "SFX".to_string(),
                ..Default::default()
            },
            Sensable {
                is_audible: true,
                always_sensed: true,
                ..Default::default()
            },
            Sfx {
                unit_db: 21.,
                stream_id: "/content/audio/ambience/spaceshipAmbientSound.sample".to_string(),
                play_back_position: 0.,
                play_back_duration: AMBIENCE_SFX_PLAY_BACK_DURATION,
                auto_destroy: false,
                ..Default::default()
            },
            EntityUpdates::default(),
            AmbienceSfxTimer::default(),
        )
    }
}
