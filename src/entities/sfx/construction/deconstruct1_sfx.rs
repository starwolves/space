use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct Deconstruct1SfxBundle;

pub const DECONSTRUCT1_PLAY_BACK_DURATION: f32 = 2. + 1.;

impl Deconstruct1SfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 11.,
            unit_size: 1.,
            stream_id: "/content/audio/construction/deconstruct1.sample".to_string(),
            play_back_duration: DECONSTRUCT1_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },));
        commands
    }
}
