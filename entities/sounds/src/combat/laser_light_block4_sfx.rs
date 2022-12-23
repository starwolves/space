use bevy::prelude::{Commands, Entity};
use resources::content::SF_CONTENT_PREFIX;
use sfx::builder::{get_random_pitch_scale, Sfx};
#[cfg(feature = "server")]
pub struct LaserLightBlock4Bundle;

#[cfg(feature = "server")]
pub const LASER_LIGHT_BLOCK4_PLAY_BACK_DURATION: f32 = 0.5 + 1.;

#[cfg(feature = "server")]
impl LaserLightBlock4Bundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn((Sfx {
                unit_db: 15.,
                unit_size: 1.,
                stream_id: SF_CONTENT_PREFIX.to_string() + "laser_light_block4",

                play_back_duration: LASER_LIGHT_BLOCK4_PLAY_BACK_DURATION,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },))
            .id()
    }
}
