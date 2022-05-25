use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct LaserLightShot4Bundle;

pub const LASER_LIGHT_SHOT4_PLAY_BACK_DURATION: f32 = 0.5 + 0.7;

impl LaserLightShot4Bundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 15.,
            unit_size: 1.,
            stream_id: "/content/audio/combat/laser_light_shot4.sample".to_string(),
            play_back_duration: LASER_LIGHT_SHOT4_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(3.),
            ..Default::default()
        },));
        commands
    }
}
