use bevy_ecs::system::Commands;
use bevy_transform::components::Transform;

use crate::space::core::{
    entity::components::{EntityData, EntityUpdates},
    static_body::components::StaticTransform,
};

use super::components::GIProbe;

pub struct GIProbeBundle;

impl GIProbeBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        _correct_transform: bool,
        gi_probe_component: GIProbe,
    ) {
        let static_transform_component = StaticTransform {
            transform: entity_transform,
        };

        commands.spawn_bundle((
            gi_probe_component,
            static_transform_component,
            EntityData {
                entity_class: "gi_probe".to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
        ));
    }
}
