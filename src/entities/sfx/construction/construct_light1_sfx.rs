use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct ConstructLight1SfxBundle;

pub const CONSTRUCTLIGHT1_PLAY_BACK_DURATION: f32 = 1.5 + 1.;

impl ConstructLight1SfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 15.,
            unit_size: 1.,
            stream_id: "/content/audio/construction/construct_light1.sample".to_string(),
            play_back_duration: CONSTRUCTLIGHT1_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },));
        commands
    }
}
