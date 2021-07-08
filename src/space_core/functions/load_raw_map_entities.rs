use bevy::{math::{Mat4, Quat, Vec3}, prelude::{BuildChildren, Commands, Transform}};
use bevy_rapier3d::prelude::{ActiveEvents, CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyCcd, RigidBodyType};


use std::collections::HashMap;

use crate::space_core::{components::{air_lock::{AccessLightsStatus, AirLock, AirLockStatus}, cached_broadcast_transform::CachedBroadcastTransform, counter_window::{CounterWindow, CounterWindowAccessLightsStatus, CounterWindowStatus}, counter_window_sensor::CounterWindowSensor, entity_data::{EntityData, EntityGroup}, entity_updates::EntityUpdates, examinable::Examinable, helmet::Helmet, inventory::SlotType, pickupable::Pickupable, sensable::Sensable, static_transform::StaticTransform, world_mode::{WorldMode,WorldModes}}, enums::space_access_enum::SpaceAccessEnum, functions::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, new_chat_message::{FURTHER_ITALIC_FONT, FURTHER_NORMAL_FONT}, string_to_type_converters::{string_transform_to_transform}, transform_to_isometry::transform_to_isometry}, process_content::entities::{gi_probe, omni_light, raw_entity::RawEntity, reflection_probe}};

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
                    updates_difference: HashMap::new(),
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
                    updates_difference: HashMap::new(),
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
                    updates_difference: HashMap::new(),
                }
            ));


        } else if raw_entity.entity_type == "securityAirLock1" {

            let static_transform_component = StaticTransform {
                transform: entity_transform
            };

            let mut entity_updates_map = HashMap::new();
            entity_updates_map.insert(".".to_string(), HashMap::new());

            let rigid_body_component = RigidBodyBundle {
                body_type: RigidBodyType::Static,
                position: transform_to_isometry(entity_transform).into(),
                ..Default::default()
            };

            let masks = get_bit_masks(ColliderGroup::Standard);

            let collider_component = ColliderBundle {
                shape: ColliderShape::cuboid(1.,1.,0.2),
                position: Vec3::new(0., 1., 0.).into(),
                flags: ColliderFlags {
                    collision_groups: InteractionGroups::new(masks.0,masks.1),
                    active_events: (ActiveEvents::CONTACT_EVENTS),
                    ..Default::default()
                },
                ..Default::default()
            };

            let examine_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]*******\n"
            + "A security air lock. It will only grant access to those authorised to use it."
            + "[font=" + FURTHER_ITALIC_FONT + "]\n\nIt is in perfect shape and fully operational.[/font]"
            + "\n*******[/font]";

            commands.spawn_bundle(rigid_body_component).insert_bundle(collider_component).insert_bundle((
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
                    updates_difference: HashMap::new(),
                },
                Examinable {
                    text: examine_text,
                }
            ));


        } else if raw_entity.entity_type == "securityCounterWindow" {

            let static_transform_component = StaticTransform {
                transform: entity_transform
            };

            let mut entity_updates_map = HashMap::new();
            entity_updates_map.insert(".".to_string(), HashMap::new());


            let window_rigid_body_component = RigidBodyBundle {
                body_type: RigidBodyType::Static,
                position: transform_to_isometry(entity_transform).into(),
                ..Default::default()
            };

            let masks = get_bit_masks(ColliderGroup::Standard);

            let window_collider_component = ColliderBundle {
                shape: ColliderShape::cuboid(0.1,0.593,1.),
                position: Vec3::new(0., -1., 1.).into(),
                flags: ColliderFlags {
                    collision_groups: InteractionGroups::new(masks.0,masks.1),
                    ..Default::default()
                },
                ..Default::default()
            };

            let sensor_rigid_body_component = RigidBodyBundle {
                body_type: RigidBodyType::Static,
                position: transform_to_isometry(entity_transform).into(),
                ..Default::default()
            };


            let masks = get_bit_masks(ColliderGroup::Standard);

            let sensor_collider_component = ColliderBundle {
                collider_type : ColliderType::Sensor,
                shape: ColliderShape::cuboid(1.,1.,1.),
                position: Vec3::new(0., -1., 1.).into(),
                flags: ColliderFlags {
                    collision_groups: InteractionGroups::new(masks.0,masks.1),
                    active_events: (ActiveEvents::INTERSECTION_EVENTS),
                    ..Default::default()
                },
                ..Default::default()
            };

            let examine_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]*******\n"
            + "An airtight security window. It will only grant access to those authorised to use it."
            + "[font=" + FURTHER_ITALIC_FONT + "]\n\nIt is in perfect shape and fully operational.[/font]"
            + "\n*******[/font]";

            let parent = commands.spawn_bundle(window_rigid_body_component).insert_bundle(window_collider_component).insert_bundle((
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
                    updates_difference: HashMap::new(),
                },
                Examinable {
                    text: examine_text,
                }
            )).id();


            let child = commands.spawn_bundle(sensor_rigid_body_component).insert_bundle(sensor_collider_component).insert_bundle((
                CounterWindowSensor {
                    parent : parent
                },
                static_transform_component,
                EntityData{
                    entity_class: "child".to_string(),
                    entity_type: "counterWindowSensor".to_string(),
                    entity_group: EntityGroup::CounterWindowSensor
                },
            )).id();

            commands.entity(parent).push_children(&[child]);


        } else if raw_entity.entity_type == "helmetSecurity" {

            let rigid_body_component = RigidBodyBundle {
                body_type: RigidBodyType::Dynamic,
                position: transform_to_isometry(entity_transform).into(),
                ccd: RigidBodyCcd {
                    ccd_enabled: false,
                    ..Default::default()
                },
                ..Default::default()
            };
    
    
            let masks = get_bit_masks(ColliderGroup::Standard);
    
            let collider_component = ColliderBundle {
                
                shape: ColliderShape::cuboid(
                    0.208,
                    0.277,
                    0.213,
                ),
                position: Vec3::new(0., 0.011, -0.004).into(),
                material: ColliderMaterial {
                    friction: 0.75,
                    friction_combine_rule:  CoefficientCombineRule::Average,
                    ..Default::default()
                },
                flags: ColliderFlags {
                    collision_groups: InteractionGroups::new(masks.0,masks.1),
                    ..Default::default()
                },
                ..Default::default()
            };
    
            let mut entity_updates_map = HashMap::new();
            entity_updates_map.insert(".".to_string(), HashMap::new());
    
            let examine_text = "[font=".to_owned() + FURTHER_NORMAL_FONT + "]*******\n"
            + "A standard issue helmet used by Security Officers."
            + "[font=" + FURTHER_ITALIC_FONT + "]\n\nIt is in perfect shape.[/font]"
            + "\n*******[/font]";
            
            let mut attachment_transforms = HashMap::new();

            attachment_transforms.insert("left_hand".to_string(), Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                Vec3::new(0.5,0.5,0.5),
              Quat::from_axis_angle(Vec3::new(1.,0.,0.), 3.111607897),
           Vec3::new(0.,-0.003, -0.108)
                )
            ));

            let right_hand_rotation = Vec3::new(0.11473795,0.775676679,0.);
            let right_hand_rotation_length = right_hand_rotation.length();

            attachment_transforms.insert("right_hand".to_string(), Transform::from_matrix(
                Mat4::from_scale_rotation_translation(
                Vec3::new(0.5,0.5,0.5),
              Quat::from_axis_angle(Vec3::new(0.11473795,0.775676679,0.).normalize(), right_hand_rotation_length),
           Vec3::new(0.064,-0.019, 0.065)
                )
            ));



            commands.spawn_bundle(rigid_body_component).insert_bundle(
                collider_component,
            ).insert_bundle((
                Sensable{
                    is_audible : false,
                    is_light:false,
                    sensed_by_cached:vec![],
                    sensed_by:vec![],
                    always_sensed : false
                },
                EntityData {
                    entity_class : "entity".to_string(),
                    entity_type : "helmetSecurity".to_string(),
                    entity_group: EntityGroup::None
                },
                EntityUpdates{
                    updates: entity_updates_map,
                    changed_parameters: vec![],
                    excluded_handles:HashMap::new(),
                    updates_difference: HashMap::new(),
                },
                WorldMode {
                    mode : WorldModes::Physics
                },
                CachedBroadcastTransform::new(),
                Examinable {
                    text: examine_text,
                },
                Helmet,
                Pickupable {
                    in_inventory_of_entity: None,
                    attachment_transforms: attachment_transforms,
                    drop_transform: Transform::from_matrix(
                     Mat4::from_scale_rotation_translation(
                Vec3::new(1.,1.,1.),
              Quat::from_axis_angle(Vec3::new(-0.0394818427,0.00003351599,1.), 3.124470974),
           Vec3::new(0.,0.355, 0.)
                    ),),
                    slot_type: SlotType::Helmet
                },
            ));

        }  else if raw_entity.entity_type == "jumpsuitSecurity" {



        }

    }

}
