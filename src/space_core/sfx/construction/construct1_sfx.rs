use bevy::prelude::Transform;

use crate::space_core::{ecs::{sfx::components::{Sfx, get_random_pitch_scale}, static_body::components::StaticTransform, entity::components::{EntityData, Sensable, EntityUpdates}}};

pub struct Construct1SfxBundle;

pub const CONSTRUCT1_PLAY_BACK_DURATION : f32 = 0.65 + 1.;

impl Construct1SfxBundle {
    
    pub fn new(passed_transform : Transform) -> (
        StaticTransform,
        EntityData,
        Sensable,
        Sfx,
        EntityUpdates
    ) {


        (StaticTransform {
            transform: passed_transform,
        },
        EntityData {
            entity_class : "SFX".to_string(),
            ..Default::default()
        },
        Sensable {
            is_audible: true,
            ..Default::default()
        },
        Sfx {
            unit_db: 15.,
            unit_size: 1.,
            stream_id: "construct1".to_string(),
            play_back_duration: CONSTRUCT1_PLAY_BACK_DURATION,
            pitch_scale: get_random_pitch_scale(1.0),
            ..Default::default()
        },
        EntityUpdates::default(),)

    }

}