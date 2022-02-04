use bevy::{prelude::{Commands, Res}};


use crate::space_core::{entities::{gi_probe::{spawn::GIProbeBundle, process_content::ExportData}, omni_light::{spawn::OmniLightBundle, self}, reflection_probe::{spawn::ReflectionProbeBundle, self}}, ecs::entity::{resources::EntityDataResource, functions::string_to_type_converters::string_transform_to_transform}};

use super::raw_entity::RawEntity;


pub fn load_raw_map_entities(
    raw_entities : &Vec<RawEntity>,
    commands : &mut Commands,
    entity_data : &Res<EntityDataResource>,
) {

    for raw_entity in raw_entities.iter() {

        let entity_transform = string_transform_to_transform(&raw_entity.transform);
        
        if raw_entity.entity_type == "OmniLight" {

            let omni_light_data_raw : omni_light::process_content::ExportDataRaw = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity OmniLight data.");
            let omni_light_component = omni_light::process_content::ExportData::new(omni_light_data_raw).to_component();

            OmniLightBundle::spawn(
                entity_transform,
                commands,
                false,
                omni_light_component
            );

        } else if raw_entity.entity_type == "GIProbe" {


            let gi_probe_data  : ExportData = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity GIProbe data.");
            let gi_probe_component = gi_probe_data.to_component();

            GIProbeBundle::spawn(
                entity_transform,
                commands,
                false,
                gi_probe_component,
            );


        } else if raw_entity.entity_type == "ReflectionProbe" {

            let reflection_probe_data_raw : reflection_probe::process_content::ExportDataRaw = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity ReflectionProbe data.");
            let reflection_probe_component = reflection_probe::process_content::ExportData::new(reflection_probe_data_raw).to_component();

            ReflectionProbeBundle::spawn(
                entity_transform,
                commands,
                false,
                reflection_probe_component,
            );

        } else {
            
            match entity_data.name_to_id.get(&raw_entity.entity_type) {
                Some(entity_type_id) => {
                    let entity_properties = entity_data.data.get(*entity_type_id).unwrap();
                    Some((*entity_properties.spawn_function)
                        (entity_transform,
                        commands,
                        false,
                        None,
                        None,
                    ));
                },
                None => {

                },
            }


        }

    }

}
