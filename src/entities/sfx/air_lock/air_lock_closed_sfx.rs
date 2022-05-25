use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct AirLockClosedSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1.5 + 1.;

impl AirLockClosedSfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 19.,
            unit_size: 1.,
            stream_id: "/content/audio/airLock/doorCloseCompression.sample".to_string(),
            play_back_duration: PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },));
        commands
    }
}
