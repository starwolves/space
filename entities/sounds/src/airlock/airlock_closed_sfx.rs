use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct AirLockClosedSfxBundle;

#[cfg(feature = "server")]
pub const PLAY_BACK_DURATION: f32 = 1.5 + 1.;

#[cfg(feature = "server")]
impl AirLockClosedSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 19.,
                unit_size: 1.,
                stream_id: "/content/audio/airLock/doorCloseCompression.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}