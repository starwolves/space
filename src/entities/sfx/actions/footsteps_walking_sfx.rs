use bevy_ecs::system::EntityCommands;

use crate::core::sfx::components::{get_random_pitch_scale, FootstepsWalking, RepeatingSfx};

pub struct FootstepsWalkingSfxBundle;

impl FootstepsWalkingSfxBundle {
    pub fn new<'w, 's, 'a>(mut commands: EntityCommands<'w, 's, 'a>) -> EntityCommands<'w, 's, 'a> {
        commands.insert_bundle((
            RepeatingSfx {
                unit_db: 12.0,
                stream_id: "concrete_walking_footsteps".to_string(),
                auto_destroy: true,
                repeat_time: 0.5,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },
            FootstepsWalking,
        ));
        commands
    }
}
