use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::Sfx;

pub struct CounterWindowDeniedSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1. + 1.;

impl CounterWindowDeniedSfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 20.,
            stream_id: "/content/audio/counterWindow/windowAccessDenied.sample".to_string(),
            play_back_duration: PLAY_BACK_DURATION,
            ..Default::default()
        },));
        commands
    }
}
