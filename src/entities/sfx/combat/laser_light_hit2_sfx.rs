use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct LaserLightHit2Bundle;

pub const LASER_LIGHT_HIT2_PLAY_BACK_DURATION: f32 = 3. + 1.;

impl LaserLightHit2Bundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((Sfx {
            unit_db: 25.,
            unit_size: 1.,
            stream_id: "/content/audio/combat/laser_light_hit2.sample".to_string(),
            play_back_duration: LASER_LIGHT_HIT2_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },));
        commands
    }
}
