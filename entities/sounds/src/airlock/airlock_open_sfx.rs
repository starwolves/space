use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct AirLockOpenSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 4.5 + 1.;

impl AirLockOpenSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 13.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "doorOpen",

                play_back_duration: PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.6),
                ..Default::default()
            },))
            .id()
    }
}
