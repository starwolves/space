use bevy::core::Timer;
use bevy::prelude::{Commands, Entity};

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
                AmbienceSfxTimer {
                    timer: Timer::from_seconds(AMBIENCE_SFX_PLAY_BACK_DURATION, false),
                },
            ))
            .id()
    }
}

use bevy::prelude::Transform;
use sfx::builder::{ambience_sfx_builder, AmbienceSfxTimer, Sfx};

pub fn startup_ambience(mut commands: Commands) {
    ambience_sfx_builder(
        &mut commands,
        Transform::identity(),
        Box::new(AmbienceSfxBundle::new),
    );
}
