use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct LaserLightShot3Bundle;

#[cfg(feature = "server")]
pub const LASER_LIGHT_SHOT3_PLAY_BACK_DURATION: f32 = 1.8 + 0.8;

#[cfg(feature = "server")]
impl LaserLightShot3Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/laser_light_shot3.sample".to_string(),
                play_back_duration: LASER_LIGHT_SHOT3_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(3.),
                ..Default::default()
            },))
            .id()
    }
}
