
use std::collections::{BTreeMap};

use bevy::{math::Vec3, prelude::{BuildChildren, Commands, Entity, Transform}};
use bevy_rapier3d::prelude::{ActiveEvents, CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyType};

use crate::space::{core::{pawn::{components::SpaceAccessEnum, functions::new_chat_message::{FURTHER_ITALIC_FONT, HEALTHY_COLOR}}, health::components::Health, static_body::components::StaticTransform, entity::{components::{Sensable, EntityData, EntityUpdates, Examinable, EntityGroup, RichName}, resources::{SpawnHeldData, SpawnPawnData}, functions::transform_to_isometry::transform_to_isometry}, physics::functions::{get_bit_masks, ColliderGroup}}};

use super::components::{CounterWindow, CounterWindowSensor};

pub struct SecurityCounterWindowBundle;

impl SecurityCounterWindowBundle {

    pub fn spawn(
        entity_transform : Transform,
        commands : &mut Commands,
        _correct_transform : bool,
        _pawn_data_option : Option<SpawnPawnData>,
        _held_data_option : Option<SpawnHeldData>
    ) -> Entity {

        let static_transform_component = StaticTransform {
            transform: entity_transform
        };


        let window_rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: transform_to_isometry(entity_transform).into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        let window_collider_component = ColliderBundle {
            shape: ColliderShape::cuboid(0.1,0.593,1.).into(),
            position: Vec3::new(0., 0., 1.).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                ..Default::default()
            }.into(),
            material: ColliderMaterial {
                friction: 0.,
                friction_combine_rule:  CoefficientCombineRule::Average,
                ..Default::default()
            }.into(),
            ..Default::default()
        };

        let sensor_rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: transform_to_isometry(entity_transform).into(),
            ..Default::default()
        };


        let masks = get_bit_masks(ColliderGroup::Standard);

        let sensor_collider_component = ColliderBundle {
            collider_type : ColliderType::Sensor.into(),
            shape: ColliderShape::cuboid(1.,1.,1.).into(),
            position: Vec3::new(0., -1., 1.).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0,masks.1),
                active_events: (ActiveEvents::INTERSECTION_EVENTS),
                ..Default::default()
            }.into(),
            ..Default::default()
        };

        let mut examine_map = BTreeMap::new();
        examine_map.insert(0,  "An airtight security window. It will only grant access to those authorised to use it.".to_string());
        examine_map.insert(1,  "[font=".to_string() + FURTHER_ITALIC_FONT + "][color=" + HEALTHY_COLOR + "]It is fully operational.[/color][/font]");

        let parent = commands.spawn_bundle(window_rigid_body_component).insert_bundle(window_collider_component).insert_bundle((
            static_transform_component,
            Sensable::default(),
            CounterWindow {
                access_permissions: vec![SpaceAccessEnum::Security],
                ..Default::default()
            },
            EntityData{
                entity_class: "entity".to_string(),
                entity_type: "securityCounterWindow".to_string(),
                entity_group: EntityGroup::AirLock
            },
            EntityUpdates::default(),
            Examinable {
                name : RichName {
                    name: "security counter window".to_string(),
                    n: false,
                    ..Default::default()
                },
                assigned_texts: examine_map,
                ..Default::default()
            },
            Health {
                is_combat_obstacle : true,
                is_laser_obstacle: false,
                is_reach_obstacle: true,
                ..Default::default()
            },
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

        parent

    }

}
