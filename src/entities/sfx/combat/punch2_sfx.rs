use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct Punch2SfxBundle;

pub const PUNCH2_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

impl Punch2SfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 12.,
            unit_size: 1.,
            stream_id: "/content/audio/combat/punch2.sample".to_string(),
            play_back_duration: PUNCH2_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },));
        commands
    }
}
