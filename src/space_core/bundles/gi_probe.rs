
use bevy::prelude::{Commands, Transform};

use crate::space_core::components::{entity_data::{EntityData}, entity_updates::EntityUpdates, gi_probe::GIProbe, static_transform::StaticTransform};

pub struct GIProbeBundle;

impl GIProbeBundle {

    pub fn spawn(
        entity_transform : Transform,
        commands : &mut Commands,
        _correct_transform : bool,
        gi_probe_component : GIProbe,
    ) {

        let static_transform_component = StaticTransform {
            transform: entity_transform
        };



        commands.spawn_bundle((
            gi_probe_component,
            static_transform_component,
            EntityData{
                entity_class: "gi_probe".to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
        ));

    }

}
