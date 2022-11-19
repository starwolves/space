use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct AirLockOpenSfxBundle;

#[cfg(feature = "server")]
pub const PLAY_BACK_DURATION: f32 = 4.5 + 1.;

#[cfg(feature = "server")]
impl AirLockOpenSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 13.,
                stream_id: "/content/audio/airLock/doorOpen.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.6),
                ..Default::default()
            },))
            .id()
    }
}
