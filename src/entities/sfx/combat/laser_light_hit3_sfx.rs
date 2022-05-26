use bevy_ecs::{entity::Entity, system::Commands};

use crate::core::sfx::components::{get_random_pitch_scale, Sfx};

pub struct LaserLightHit3Bundle;

pub const LASER_LIGHT_HIT3_PLAY_BACK_DURATION: f32 = 1.8 + 1.;

impl LaserLightHit3Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 25.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/laser_light_hit3.sample".to_string(),
                play_back_duration: LASER_LIGHT_HIT3_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
