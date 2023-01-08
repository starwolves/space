use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct ConstructLight2SfxBundle;

pub const CONSTRUCTLIGHT2_PLAY_BACK_DURATION: f32 = 1.9 + 1.;

impl ConstructLight2SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "construct_light2",

                play_back_duration: CONSTRUCTLIGHT2_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
