use bevy::prelude::{Commands, Entity};
use sfx::builder::{get_random_pitch_scale, FootstepsSprinting, RepeatingSfx};

#[cfg(feature = "server")]
pub struct FootstepsSprintingSfxBundle;

#[cfg(feature = "server")]
impl FootstepsSprintingSfxBundle {
    pub fn new(commands: &mut Commands) -> Entity {
        commands
            .spawn_bundle((
                RepeatingSfx {
                    unit_db: 12.0,
                    unit_size: 1.,
                    stream_id: "concrete_sprinting_footsteps".to_string(),
                    auto_destroy: true,
                    repeat_time: 0.35,
                    pitch_scale: get_random_pitch_scale(1.0),
                    ..Default::default()
                },
                FootstepsSprinting,
            ))
            .id()
    }
}
