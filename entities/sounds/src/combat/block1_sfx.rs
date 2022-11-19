use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct Block1SfxBundle;

#[cfg(feature = "server")]
pub const BLOCK1_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

#[cfg(feature = "server")]
impl Block1SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/block1.sample".to_string(),
                play_back_duration: BLOCK1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
