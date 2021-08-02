
use bevy::prelude::{Commands, Transform};

use crate::space_core::{components::{entity_data::{EntityData}, entity_updates::EntityUpdates, omni_light::OmniLight, sensable::Sensable, static_transform::StaticTransform, world_mode::{WorldMode, WorldModes}}};

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


        commands.spawn_bundle((
            omni_light_component,
            Sensable{
                is_light:true,
                ..Default::default()
            },
            static_transform_component,
            EntityData{
                entity_class: "omni_light".to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            WorldMode {
                mode : WorldModes::Static
            }
        ));

    }

}
