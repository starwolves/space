use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct LaserLightHit4Bundle;

#[cfg(feature = "server")]
pub const LASER_LIGHT_HIT4_PLAY_BACK_DURATION: f32 = 2.1 + 1.;

#[cfg(feature = "server")]
impl LaserLightHit4Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 25.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/laser_light_hit4.sample".to_string(),
                play_back_duration: LASER_LIGHT_HIT4_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
