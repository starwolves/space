use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bevy_ecs::{entity::Entity, system::Commands};
use bevy_hierarchy::BuildChildren;
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{
    ActiveEvents, CoefficientCombineRule, Collider, CollisionGroups, Friction, RigidBody, Sensor,
};
use bevy_transform::components::Transform;

use crate::{
    core::{
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::{
            components::{DefaultMapEntity, EntityData, EntityGroup, EntityUpdates},
            resources::{SpawnHeldData, SpawnPawnData},
        },
        examinable::components::{Examinable, RichName},
        health::components::Health,
        networking::resources::ConsoleCommandVariantValues,
        pawn::components::ShipAuthorizationEnum,
        physics::functions::{get_bit_masks, ColliderGroup},
        sensable::components::Sensable,
        static_body::components::StaticTransform,
        tab_actions::components::{TabAction, TabActions},
    },
    entities::counter_windows::components::{CounterWindow, CounterWindowSensor},
};

use super::functions::{lock_closed_action, lock_open_action, toggle_open_action, unlock_action};

pub struct CounterWindowBundle;

impl CounterWindowBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        _correct_transform: bool,
        _pawn_data_option: Option<SpawnPawnData>,
        _held_data_option: Option<SpawnHeldData>,
        default_map_spawn: bool,
        _properties: HashMap<String, ConsoleCommandVariantValues>,
    ) -> Entity {
        let static_transform_component = StaticTransform {
            transform: entity_transform,
        };

        let rigid_body = RigidBody::Fixed;

        let masks = get_bit_masks(ColliderGroup::Standard);

        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Average;

        let mut parent_builder = commands.spawn();
        parent_builder.insert(rigid_body).insert(entity_transform);
        let parent = parent_builder.id();

        parent_builder
            .insert(Collider::cuboid(0.1, 0.593, 1.))
            .insert(Transform::from_translation(Vec3::new(0., 0., 1.)))
            .insert(friction)
            .insert(CollisionGroups::new(masks.0, masks.1));

        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "An airtight security window. It will only grant access to those authorised to use it."
                .to_string(),
        );
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );

        parent_builder.insert_bundle((
            static_transform_component,
            Sensable::default(),
            CounterWindow {
                access_permissions: vec![ShipAuthorizationEnum::Security],
                ..Default::default()
            },
            EntityData {
                entity_class: "entity".to_string(),
                entity_name: "securityCounterWindow".to_string(),
                entity_group: EntityGroup::AirLock,
            },
            EntityUpdates::default(),
            Examinable {
                name: RichName {
                    name: "security counter window".to_string(),
                    n: false,
                    ..Default::default()
                },
                assigned_texts: examine_map,
                ..Default::default()
            },
            Health {
                is_combat_obstacle: true,
                is_laser_obstacle: false,
                is_reach_obstacle: true,
                ..Default::default()
            },
            TabActions {
                tab_actions: vec![
                    TabAction {
                        id: "actions::counter_windows/toggleopen".to_string(),
                        text: "Toggle Open".to_string(),
                        tab_list_priority: 100,
                        prerequisite_check: Arc::new(toggle_open_action),
                        belonging_entity: Some(parent),
                    },
                    TabAction {
                        id: "actions::counter_windows/lockopen".to_string(),
                        text: "Lock Open".to_string(),
                        tab_list_priority: 99,
                        prerequisite_check: Arc::new(lock_open_action),
                        belonging_entity: Some(parent),
                    },
                    TabAction {
                        id: "actions::counter_windows/lockclosed".to_string(),
                        text: "Lock Closed".to_string(),
                        tab_list_priority: 98,
                        prerequisite_check: Arc::new(lock_closed_action),
                        belonging_entity: Some(parent),
                    },
                    TabAction {
                        id: "actions::counter_windows/unlock".to_string(),
                        text: "Unlock".to_string(),
                        tab_list_priority: 97,
                        prerequisite_check: Arc::new(unlock_action),
                        belonging_entity: Some(parent),
                    },
                ],
            },
        ));

        if default_map_spawn {
            parent_builder.insert(DefaultMapEntity);
        }

        let rigid_body = RigidBody::Fixed;

        let masks = get_bit_masks(ColliderGroup::Standard);

        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Average;

        let sensor = Sensor(true);

        let mut sensor_builder = commands.spawn();
        sensor_builder.insert(rigid_body).insert(entity_transform);
        sensor_builder
            .insert(Collider::cuboid(1., 1., 1.))
            .insert(Transform::from_translation(Vec3::new(0., -1., 1.)))
            .insert(friction)
            .insert(CollisionGroups::new(masks.0, masks.1))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(sensor);

        let child = sensor_builder
            .insert_bundle((
                CounterWindowSensor { parent: parent },
                static_transform_component,
                EntityData {
                    entity_class: "child".to_string(),
                    entity_name: "counterWindowSensor".to_string(),
                    entity_group: EntityGroup::CounterWindowSensor,
                },
            ))
            .id();

        commands.entity(parent).push_children(&[child]);

        parent
    }
}
