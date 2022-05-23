use bevy_transform::components::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    sensable::components::Sensable,
    sfx::components::{get_random_pitch_scale, Sfx},
};

pub struct Swing1SfxBundle;

pub const SWING1_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Swing1SfxBundle {
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
                unit_db: 12.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/swing1.sample".to_string(),
                play_back_duration: SWING1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },
            EntityUpdates::default(),
        )
    }
}
