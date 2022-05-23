use bevy_ecs::system::Commands;
use bevy_transform::components::Transform;

use crate::core::{
    entity::components::{EntityData, EntityUpdates},
    physics::components::{WorldMode, WorldModes},
    sensable::components::Sensable,
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
        let static_transform_component = entity_transform;

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
