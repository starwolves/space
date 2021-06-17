use bevy::{prelude::{BuildChildren, Commands}};
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

use super::raw_entity::RawEntity;

use std::collections::HashMap;

use crate::space_core::{components::{air_lock::{AccessLightsStatus, AirLock, AirLockStatus}, counter_window::{CounterWindow, CounterWindowAccessLightsStatus, CounterWindowStatus}, counter_window_sensor::CounterWindowSensor, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, sensable::Sensable, static_transform::StaticTransform, world_mode::{WorldMode,WorldModes}}, enums::space_access_enum::SpaceAccessEnum, functions::{string_to_type_converters::{string_transform_to_transform}, transform_to_isometry::transform_to_isometry}, process_content::entities::{
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
                    updates: entity_updates_map,
                    changed_parameters: vec![],
                    excluded_handles: HashMap::new(),
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
                    updates: entity_updates_map,
                    changed_parameters: vec![],
                    excluded_handles: HashMap::new(),
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
                Sensable{
                    is_audible : false,
                    is_light:false,
                    sensed_by: vec![],
                    sensed_by_cached: vec![],
                    always_sensed : false
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
                    updates: entity_updates_map,
                    changed_parameters: vec![],
                    excluded_handles:HashMap::new(),
                }
            ));


        } else if raw_entity.entity_type == "securityCounterWindow" {

            let static_transform_component = StaticTransform {
                transform: entity_transform
            };

            let mut entity_updates_map = HashMap::new();
            entity_updates_map.insert(".".to_string(), HashMap::new());

            let window_rigid_body_component = RigidBodyBuilder::new_static()
            .ccd_enabled(true)
            .position(transform_to_isometry(entity_transform));

            let window_collider_component = ColliderBuilder::cuboid(0.1,0.593,1.)
            .translation(0., -1., 1.);


            let sensor_rigid_body_component = RigidBodyBuilder::new_static()
            .ccd_enabled(true)
            .position(transform_to_isometry(entity_transform));

            let sensor_collider_component = ColliderBuilder::cuboid(1.,1.,1.)
            .translation(0., -1., 1.)
            .sensor(true);

            

            let parent = commands.spawn_bundle((
                window_rigid_body_component,
                window_collider_component,
                static_transform_component,
                Sensable{
                    is_audible : false,
                    is_light:false,
                    sensed_by: vec![],
                    sensed_by_cached: vec![],
                    always_sensed : false
                },
                CounterWindow {
                    status: CounterWindowStatus::Closed,
                    access_lights: CounterWindowAccessLightsStatus::Neutral,
                    access_permissions: vec![SpaceAccessEnum::Security],
                },
                EntityData{
                    entity_class: "entity".to_string(),
                    entity_type: "securityCounterWindow".to_string(),
                    entity_group: EntityGroup::AirLock
                },
                EntityUpdates{
                    updates: entity_updates_map,
                    changed_parameters: vec![],
                    excluded_handles:HashMap::new(),
                }
            )).id();


            let child = commands.spawn_bundle((
                CounterWindowSensor {
                    parent : parent
                },
                static_transform_component,
                sensor_rigid_body_component,
                sensor_collider_component,
                EntityData{
                    entity_class: "child".to_string(),
                    entity_type: "counterWindowSensor".to_string(),
                    entity_group: EntityGroup::CounterWindowSensor
                },
            )).id();

            commands.entity(parent).push_children(&[child]);


        }

    }

}
