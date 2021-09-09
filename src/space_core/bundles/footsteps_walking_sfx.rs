
use bevy::prelude::{Transform};

use crate::space_core::components::{cached_broadcast_transform::CachedBroadcastTransform, entity_data::{EntityData}, entity_updates::EntityUpdates, footsteps_walking::FootstepsWalking, repeating_sfx::RepeatingSfx, sensable::Sensable, sfx::get_random_pitch_scale, static_transform::StaticTransform, update_transform::UpdateTransform};

pub struct FootstepsWalkingSfxBundle;

impl FootstepsWalkingSfxBundle {

    pub fn new(passed_transform : Transform) -> (
        StaticTransform,
        EntityData,
        Sensable,
        RepeatingSfx,
        EntityUpdates,
        FootstepsWalking,
        UpdateTransform,
        CachedBroadcastTransform
    ) {


        (StaticTransform {
            transform: passed_transform,
        },
        EntityData {
            entity_class : "RepeatingSFX".to_string(),
            ..Default::default()
        },
        Sensable {
            is_audible: true,
            ..Default::default()
        },
        RepeatingSfx {
            unit_db: 12.0,
            stream_id: "concrete_walking_footsteps".to_string(),
            auto_destroy : true,
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
