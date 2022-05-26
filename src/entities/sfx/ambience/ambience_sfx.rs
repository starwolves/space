use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::sfx::components::{AmbienceSfxTimer, Sfx};

pub struct AmbienceSfxBundle;

pub const AMBIENCE_SFX_PLAY_BACK_DURATION: f32 = 424. + 1.;

impl AmbienceSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((
                Sfx {
                    unit_db: 21.,
                    stream_id: "/content/audio/ambience/spaceshipAmbientSound.sample".to_string(),
                    play_back_position: 0.,
                    play_back_duration: AMBIENCE_SFX_PLAY_BACK_DURATION,
                    auto_destroy: false,
                    ..Default::default()
                },
                AmbienceSfxTimer::default(),
            ))
            .id()
    }
}
