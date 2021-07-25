use bevy::{ prelude::{Commands}};


use crate::space_core::{bundles::{gi_probe::GIProbeBundle, helmet_security::HelmetSecurityBundle, jumpsuit_security::JumpsuitSecurityBundle, omni_light::OmniLightBundle, reflection_probe::ReflectionProbeBundle, security_airlock::SecurityAirlockBundle, security_counter_window::SecurityCounterWindowBundle}, functions::{converters::{string_to_type_converters::string_transform_to_transform}, process_content::{gi_probe, omni_light, reflection_probe}}};

use super::raw_entity::RawEntity;


pub fn load_raw_map_entities(
    raw_entities : &Vec<RawEntity>,
    commands : &mut Commands
) {

    for raw_entity in raw_entities.iter() {

        

        let entity_transform = string_transform_to_transform(&raw_entity.transform);
        
        if raw_entity.entity_type == "OmniLight" {

            let omni_light_data_raw : omni_light::ExportDataRaw = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity OmniLight data.");
            let omni_light_component = omni_light::ExportData::new(omni_light_data_raw).to_component();

            OmniLightBundle::spawn(
                entity_transform,
                commands,
                false,
                omni_light_component
            );

            

        } else if raw_entity.entity_type == "GIProbe" {


            let gi_probe_data  : gi_probe::ExportData = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity GIProbe data.");
            let gi_probe_component = gi_probe_data.to_component();

            GIProbeBundle::spawn(
                entity_transform,
                commands,
                false,
                gi_probe_component,
            );


        } else if raw_entity.entity_type == "ReflectionProbe" {

            let reflection_probe_data_raw : reflection_probe::ExportDataRaw = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity ReflectionProbe data.");
            let reflection_probe_component = reflection_probe::ExportData::new(reflection_probe_data_raw).to_component();

            ReflectionProbeBundle::spawn(
                entity_transform,
                commands,
                false,
                reflection_probe_component,
            );

        } else if raw_entity.entity_type == "securityAirLock1" {

            SecurityAirlockBundle::spawn(
                entity_transform,
                commands,
                false
            );

        } else if raw_entity.entity_type == "securityCounterWindow" {

            SecurityCounterWindowBundle::spawn(
                entity_transform,
                commands,
                false
            );

        } else if raw_entity.entity_type == "helmetSecurity" {

            HelmetSecurityBundle::spawn(entity_transform, commands, false);

        }  else if raw_entity.entity_type == "jumpsuitSecurity" {

            JumpsuitSecurityBundle::spawn(entity_transform, commands, false);

        }

    }

}
