use bevy::prelude::{Commands, Entity};
use resources::core::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct CounterWindowOpenSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1.75 + 1.;

impl CounterWindowOpenSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.0,
                stream_id: SF_CONTENT_PREFIX.to_string() + "windowOpen",
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
