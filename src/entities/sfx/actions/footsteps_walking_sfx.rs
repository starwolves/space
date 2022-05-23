use bevy_transform::components::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    rigid_body::components::{CachedBroadcastTransform, UpdateTransform},
    sensable::components::Sensable,
    sfx::components::{get_random_pitch_scale, FootstepsWalking, RepeatingSfx},
};

pub struct FootstepsWalkingSfxBundle;

impl FootstepsWalkingSfxBundle {
    pub fn new(
        passed_transform: Transform,
    ) -> (
        Transform,
        EntityData,
        Sensable,
        RepeatingSfx,
        EntityUpdates,
        FootstepsWalking,
        UpdateTransform,
        CachedBroadcastTransform,
    ) {
        (
            passed_transform,
            EntityData {
                entity_class: "RepeatingSFX".to_string(),
                ..Default::default()
            },
            Sensable {
                is_audible: true,
                ..Default::default()
            },
            RepeatingSfx {
                unit_db: 12.0,
                stream_id: "concrete_walking_footsteps".to_string(),
                auto_destroy: true,
                repeat_time: 0.5,
                pitch_scale: get_random_pitch_scale(1.0),
                ..Default::default()
            },
            EntityUpdates::default(),
            FootstepsWalking,
            UpdateTransform,
            CachedBroadcastTransform::default(),
        )
    }
}
