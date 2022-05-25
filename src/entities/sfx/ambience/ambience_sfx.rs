use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{AmbienceSfxTimer, Sfx};

pub struct AmbienceSfxBundle;

pub const AMBIENCE_SFX_PLAY_BACK_DURATION: f32 = 424. + 1.;

impl AmbienceSfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((
            Sfx {
                unit_db: 21.,
                stream_id: "/content/audio/ambience/spaceshipAmbientSound.sample".to_string(),
                play_back_position: 0.,
                play_back_duration: AMBIENCE_SFX_PLAY_BACK_DURATION,
                auto_destroy: false,
                ..Default::default()
            },
            AmbienceSfxTimer::default(),
        ));
        commands
    }
}
