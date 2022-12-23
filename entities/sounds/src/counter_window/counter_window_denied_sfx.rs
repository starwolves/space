use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct CounterWindowDeniedSfxBundle;

#[cfg(feature = "server")]
pub const PLAY_BACK_DURATION: f32 = 1. + 1.;

#[cfg(feature = "server")]
impl CounterWindowDeniedSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 20.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "windowAccessDenied",

                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
