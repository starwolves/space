use std::collections::HashMap;

use bevy::prelude::{Commands, Transform};

use crate::space_core::components::{entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, gi_probe::GIProbe, static_transform::StaticTransform};

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


        let mut entity_updates_map = HashMap::new();
            entity_updates_map.insert(".".to_string(), HashMap::new());

        commands.spawn_bundle((
            gi_probe_component,
            static_transform_component,
            EntityData{
                entity_class: "gi_probe".to_string(),
                entity_type: "".to_string(),
                entity_group: EntityGroup::None
            },
            EntityUpdates{
                updates: entity_updates_map,
                changed_parameters: vec![],
                excluded_handles: HashMap::new(),
                updates_difference: HashMap::new(),
            }
        ));

    }

}
