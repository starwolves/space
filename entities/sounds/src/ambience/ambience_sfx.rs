use bevy::prelude::{Commands, Entity};

#[cfg(feature = "server")]
pub struct AmbienceSfxBundle;

#[cfg(feature = "server")]
pub const AMBIENCE_SFX_PLAY_BACK_DURATION: f32 = 424. + 1.;

#[cfg(feature = "server")]
impl AmbienceSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Sfx {
                    unit_db: 21.,
                    stream_id: SF_CONTENT_PREFIX.to_string() + "spaceshipAmbientSound",

                    play_back_position: 0.,
                    play_back_duration: AMBIENCE_SFX_PLAY_BACK_DURATION,
                    auto_destroy: false,
                    ..Default::default()
                },
                AmbienceSfxTimer {
                    timer: Timer::from_seconds(AMBIENCE_SFX_PLAY_BACK_DURATION, TimerMode::Once),
                },
            ))
            .id()
    }
}

use bevy::prelude::Transform;
use bevy::time::{Timer, TimerMode};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{spawn_ambience_sfx, AmbienceSfxTimer, Sfx};

#[cfg(feature = "server")]
pub fn startup_ambience(mut commands: Commands) {
    spawn_ambience_sfx(
        &mut commands,
        Transform::IDENTITY,
        Box::new(AmbienceSfxBundle::new),
    );
}
