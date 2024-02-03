use bevy::prelude::{Commands, Entity};
use resources::core::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct Construct2SfxBundle;

pub const CONSTRUCT2_PLAY_BACK_DURATION: f32 = 0.7 + 1.;

impl Construct2SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 11.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "construct2",
                play_back_duration: CONSTRUCT2_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
