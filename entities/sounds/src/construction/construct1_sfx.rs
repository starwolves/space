use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct Construct1SfxBundle;

#[cfg(feature = "server")]
pub const CONSTRUCT1_PLAY_BACK_DURATION: f32 = 0.65 + 1.;

#[cfg(feature = "server")]
impl Construct1SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 11.,
                unit_size: 1.,
                stream_id: "/content/audio/construction/construct1.sample".to_string(),
                play_back_duration: CONSTRUCT1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
