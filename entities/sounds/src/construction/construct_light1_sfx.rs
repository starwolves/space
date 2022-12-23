use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct ConstructLight1SfxBundle;

#[cfg(feature = "server")]
pub const CONSTRUCTLIGHT1_PLAY_BACK_DURATION: f32 = 1.5 + 1.;

#[cfg(feature = "server")]
impl ConstructLight1SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "construct_light1",
                play_back_duration: CONSTRUCTLIGHT1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
