use bevy::prelude::{Commands, Entity};

use crate::core::sfx::builder::{get_random_pitch_scale, Sfx};

pub struct CounterWindowClosedSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1.3 + 1.;

impl CounterWindowClosedSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 18.,
                stream_id: "/content/audio/counterWindow/windowCloseCompressed.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
