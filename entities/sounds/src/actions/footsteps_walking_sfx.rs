use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, FootstepsWalking, RepeatingSfx};

#[cfg(feature = "server")]
pub struct FootstepsWalkingSfxBundle;

#[cfg(feature = "server")]
impl FootstepsWalkingSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((
                RepeatingSfx {
                    unit_db: 12.0,
                    stream_id: "concrete_walking_footsteps".to_string(),
                    auto_destroy: true,
                    repeat_time: 0.5,
                    pitch_scale: get_random_pitch_scale(1.0),
                    ..Default::default()
                },
                FootstepsWalking,
            ))
            .id()
    }
}
