use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct Punch4SfxBundle;

pub const PUNCH4_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Punch4SfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 12.,
            unit_size: 1.,
            stream_id: "/content/audio/combat/punch4.sample".to_string(),
            play_back_duration: PUNCH4_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },));
        commands
    }
}
