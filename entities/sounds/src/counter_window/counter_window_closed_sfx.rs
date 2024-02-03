use bevy::prelude::{Commands, Entity};
use resources::core::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct CounterWindowClosedSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1.3 + 1.;

impl CounterWindowClosedSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 18.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "windowCloseCompressed",
                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
