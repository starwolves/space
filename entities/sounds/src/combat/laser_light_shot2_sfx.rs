use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, Sfx};

#[cfg(feature = "server")]
pub struct LaserLightShot2Bundle;

#[cfg(feature = "server")]
pub const LASER_LIGHT_SHOT2_PLAY_BACK_DURATION: f32 = 3. + 0.7;

#[cfg(feature = "server")]
impl LaserLightShot2Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: "/content/audio/combat/laser_light_shot2.sample".to_string(),
                play_back_duration: LASER_LIGHT_SHOT2_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(3.),
                ..Default::default()
            },))
            .id()
    }
}
