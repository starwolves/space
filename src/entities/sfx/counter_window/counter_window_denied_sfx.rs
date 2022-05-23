use bevy_transform::components::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    sensable::components::Sensable,
    sfx::components::Sfx,
};

pub struct CounterWindowDeniedSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1. + 1.;

impl CounterWindowDeniedSfxBundle {
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
                unit_db: 20.,
                stream_id: "/content/audio/counterWindow/windowAccessDenied.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                ..Default::default()
            },
            EntityUpdates::default(),
        )
    }
}
