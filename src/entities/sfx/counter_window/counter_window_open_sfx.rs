use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct CounterWindowOpenSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1.75 + 1.;

impl CounterWindowOpenSfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 15.0,
            stream_id: "/content/audio/counterWindow/windowOpen.sample".to_string(),
            play_back_duration: PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },));
        commands
    }
}
