use bevy::prelude::Commands;

use super::raw_entity::RawEntity;

use crate::space_core::{
    functions::string_to_type_converters::{string_transform_to_transform},
    components::{
        visible::Visible,
        static_transform::StaticTransform
    },
    process_content::entities::{
        omni_light,
        gi_probe,
        reflection_probe
    }
};

pub fn load_raw_map_entities(raw_entities : &Vec<RawEntity>, commands : &mut Commands) {

    for raw_entity in raw_entities.iter() {

        let static_transform_component = StaticTransform {
            transform: string_transform_to_transform(&raw_entity.transform)
        };
        
        if raw_entity.entity_type == "OmniLight" {

            let omni_light_data_raw : omni_light::ExportDataRaw = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity OmniLight data.");
            let omni_light_component = omni_light::ExportData::new(omni_light_data_raw).to_component();

            commands.spawn_bundle((omni_light_component, Visible{is_light:true}, static_transform_component));

        } else if raw_entity.entity_type == "GIProbe" {

            let gi_probe_data  : gi_probe::ExportData = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity GIProbe data.");
            let gi_probe_component = gi_probe_data.to_component();

            commands.spawn_bundle((gi_probe_component,static_transform_component));


        } else if raw_entity.entity_type == "ReflectionProbe" {
            
            let reflection_probe_data_raw : reflection_probe::ExportDataRaw = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity ReflectionProbe data.");
            let reflection_probe_component = reflection_probe::ExportData::new(reflection_probe_data_raw).to_component();

            commands.spawn_bundle((reflection_probe_component,static_transform_component));


        }

    }

}
