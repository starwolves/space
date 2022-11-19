use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct ConstructLight2SfxBundle;

#[cfg(feature = "server")]
pub const CONSTRUCTLIGHT2_PLAY_BACK_DURATION: f32 = 1.9 + 1.;

#[cfg(feature = "server")]
impl ConstructLight2SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: "/content/audio/construction/construct_light2.sample".to_string(),
                play_back_duration: CONSTRUCTLIGHT2_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
