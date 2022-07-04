use bevy::prelude::{Commands, Entity};

use crate::core::sfx::builder::{get_random_pitch_scale, Sfx};

pub struct CounterWindowOpenSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1.75 + 1.;

impl CounterWindowOpenSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 15.0,
                stream_id: "/content/audio/counterWindow/windowOpen.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
