use std::collections::HashMap;

use bevy::prelude::{Commands, Transform};

use crate::space_core::{components::{entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, omni_light::OmniLight, sensable::Sensable, static_transform::StaticTransform, world_mode::{WorldMode, WorldModes}}};

pub struct OmniLightBundle;

impl OmniLightBundle {

    pub fn spawn(
        entity_transform : Transform,
        commands : &mut Commands,
        _correct_transform : bool,
        omni_light_component : OmniLight,
    ) {

        let static_transform_component = StaticTransform {
            transform: entity_transform
        };

        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());

        commands.spawn_bundle((
            omni_light_component,
            Sensable{
                is_light:true,
                is_audible : false,
                sensed_by: vec![],
                sensed_by_cached: vec![],
                always_sensed : false
            },
            static_transform_component,
            EntityData{
                entity_class: "omni_light".to_string(),
                entity_type: "".to_string(),
                entity_group: EntityGroup::None
            },
            EntityUpdates{
                updates: entity_updates_map,
                changed_parameters: vec![],
                excluded_handles: HashMap::new(),
                updates_difference: HashMap::new(),
            },
            WorldMode {
                mode : WorldModes::Static
            }
        ));

    }

}
