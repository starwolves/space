use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

pub struct Punch4SfxBundle;

pub const PUNCH4_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Punch4SfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 12.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/punch4.sample".to_string(),
                play_back_duration: PUNCH4_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
