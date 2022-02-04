
use bevy::prelude::{Transform};

use crate::space_core::{generics::{sfx::components::{RepeatingSfx, FootstepsWalking, get_random_pitch_scale}, rigid_body::components::{CachedBroadcastTransform, UpdateTransform}, static_body::components::StaticTransform, entity::components::{EntityData, Sensable, EntityUpdates}}};

pub struct FootstepsSprintingSfxBundle;

impl FootstepsSprintingSfxBundle {

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
            unit_size: 1.,
            stream_id: "concrete_sprinting_footsteps".to_string(),
            auto_destroy : true,
            repeat_time: 0.35,
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
