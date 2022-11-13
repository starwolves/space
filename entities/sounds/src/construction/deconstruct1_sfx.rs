use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct Deconstruct1SfxBundle;

#[cfg(feature = "server")]
pub const DECONSTRUCT1_PLAY_BACK_DURATION: f32 = 2. + 1.;

#[cfg(feature = "server")]
impl Deconstruct1SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 11.,
                unit_size: 1.,
                stream_id: "/content/audio/construction/deconstruct1.sample".to_string(),
                play_back_duration: DECONSTRUCT1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
