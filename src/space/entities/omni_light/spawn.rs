use bevy_internal::prelude::{Commands, Transform};

use crate::space::core::{
    entity::components::{EntityData, EntityUpdates, Sensable},
    physics::components::{WorldMode, WorldModes},
    static_body::components::StaticTransform,
};

use super::components::OmniLight;

pub struct OmniLightBundle;

impl OmniLightBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        _correct_transform: bool,
        omni_light_component: OmniLight,
    ) {
        let static_transform_component = StaticTransform {
            transform: entity_transform,
        };

        commands.spawn_bundle((
            omni_light_component,
            Sensable {
                is_light: true,
                ..Default::default()
            },
            static_transform_component,
            EntityData {
                entity_class: "omni_light".to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}
