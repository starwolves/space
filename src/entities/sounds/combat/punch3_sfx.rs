use bevy::prelude::{Commands, Entity};

use crate::core::sfx::builder::{get_random_pitch_scale, Sfx};

pub struct Punch3SfxBundle;

pub const PUNCH3_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Punch3SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 12.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/punch3.sample".to_string(),
                play_back_duration: PUNCH3_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
