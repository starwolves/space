use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct Punch2SfxBundle;

pub const PUNCH2_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Punch2SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 12.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/punch2.sample".to_string(),
                play_back_duration: PUNCH2_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
