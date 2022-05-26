use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct LaserLightBlock3Bundle;

pub const LASER_LIGHT_BLOCK3_PLAY_BACK_DURATION: f32 = 0.7 + 1.;

impl LaserLightBlock3Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/laser_light_block3.sample".to_string(),
                play_back_duration: LASER_LIGHT_BLOCK3_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
