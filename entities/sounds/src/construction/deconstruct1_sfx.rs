use bevy::prelude::{Commands, Entity};
use resources::core::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct Deconstruct1SfxBundle;

pub const DECONSTRUCT1_PLAY_BACK_DURATION: f32 = 2. + 1.;

impl Deconstruct1SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 11.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "deconstruct1",
                play_back_duration: DECONSTRUCT1_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
