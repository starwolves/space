use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct Construct1SfxBundle;

pub const CONSTRUCT1_PLAY_BACK_DURATION: f32 = 0.65 + 1.;

impl Construct1SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 11.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "construct1",
                play_back_duration: CONSTRUCT1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
