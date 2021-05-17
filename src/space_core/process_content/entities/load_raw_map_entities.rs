use bevy::{prelude::Commands};
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

use super::raw_entity::RawEntity;

use std::collections::HashMap;

use crate::space_core::{components::{air_lock::{AccessLightsStatus, AirLock, AirLockStatus}, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, static_transform::StaticTransform, visible::Visible, world_mode::{WorldMode,WorldModes}}, enums::space_access_enum::SpaceAccessEnum, functions::{string_to_type_converters::{string_transform_to_transform}, transform_to_isometry::transform_to_isometry}, process_content::entities::{
        omni_light,
        gi_probe,
        reflection_probe
    }};

pub fn load_raw_map_entities(
    raw_entities : &Vec<RawEntity>,
    commands : &mut Commands
) {

    for raw_entity in raw_entities.iter() {

        

        let entity_transform = string_transform_to_transform(&raw_entity.transform);
        
        if raw_entity.entity_type == "OmniLight" {

            let static_transform_component = StaticTransform {
                transform: entity_transform
            };

            let omni_light_data_raw : omni_light::ExportDataRaw = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity OmniLight data.");
            let omni_light_component = omni_light::ExportData::new(omni_light_data_raw).to_component();

            let mut entity_updates_map = HashMap::new();
            entity_updates_map.insert(".".to_string(), HashMap::new());

            commands.spawn_bundle((
                omni_light_component,
                Visible{
                    is_light:true,
                    sensed_by: vec![],
                    sensed_by_cached: vec![]
                },
                static_transform_component,
                EntityData{
                    entity_class: "omni_light".to_string(),
                    entity_type: "".to_string(),
                    entity_group: EntityGroup::None
                },
                EntityUpdates{
                    updates: entity_updates_map
                },
                WorldMode {
                    mode : WorldModes::Static
                }
            ));

        } else if raw_entity.entity_type == "GIProbe" {

            let static_transform_component = StaticTransform {
                transform: entity_transform
            };

            let gi_probe_data  : gi_probe::ExportData = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity GIProbe data.");
            let gi_probe_component = gi_probe_data.to_component();

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
                    updates: entity_updates_map
                }
            ));


        } else if raw_entity.entity_type == "ReflectionProbe" {
            
            let static_transform_component = StaticTransform {
                transform: entity_transform
            };

            let reflection_probe_data_raw : reflection_probe::ExportDataRaw = serde_json::from_str(&raw_entity.data).expect("load_raw_map_entities.rs Error parsing entity ReflectionProbe data.");
            let reflection_probe_component = reflection_probe::ExportData::new(reflection_probe_data_raw).to_component();

            let mut entity_updates_map = HashMap::new();
            entity_updates_map.insert(".".to_string(), HashMap::new());

            commands.spawn_bundle((
                reflection_probe_component,
                static_transform_component,
                EntityData{
                    entity_class: "reflection_probe".to_string(),
                    entity_type: "".to_string(),
                    entity_group: EntityGroup::None
                },
                EntityUpdates{
                    updates: entity_updates_map
                }
            ));


        } else if raw_entity.entity_type == "securityAirLock1" {

            let static_transform_component = StaticTransform {
                transform: entity_transform
            };

            let mut entity_updates_map = HashMap::new();
            entity_updates_map.insert(".".to_string(), HashMap::new());

            let rigid_body_component = RigidBodyBuilder::new_static()
            .ccd_enabled(true)
            .position(transform_to_isometry(entity_transform));

            let collider_component = ColliderBuilder::cuboid(1.,0.2,1.)
            .translation(0., 1., 1.);

            commands.spawn_bundle((
                rigid_body_component,
                collider_component,
                static_transform_component,
                Visible{
                    is_light:false,
                    sensed_by: vec![],
                    sensed_by_cached: vec![]
                },
                AirLock {
                    status : AirLockStatus::Closed,
                    access_lights : AccessLightsStatus::Neutral,
                    access_permissions : vec![SpaceAccessEnum::Security]
                },
                EntityData{
                    entity_class: "entity".to_string(),
                    entity_type: "securityAirLock1".to_string(),
                    entity_group: EntityGroup::AirLock
                },
                EntityUpdates{
                    updates: entity_updates_map
                }
            ));


        }

    }

}
