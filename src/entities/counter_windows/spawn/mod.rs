pub mod entity_bundle;
pub mod rigidbody_bundle;

use std::sync::Arc;

use bevy_ecs::entity::Entity;
use bevy_hierarchy::BuildChildren;
use bevy_log::warn;
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{
    ActiveEvents, CoefficientCombineRule, Collider, CollisionGroups, Friction, RigidBody, Sensor,
};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{
        components::{EntityData, EntityGroup},
        resources::SpawnData,
        spawn::{base_entity_builder, BaseEntityData},
    },
    health::components::Health,
    pawn::components::ShipAuthorizationEnum,
    physics::functions::{get_bit_masks, ColliderGroup},
    rigid_body::spawn::{rigidbody_builder, RigidBodySpawnData},
    tab_actions::components::{TabAction, TabActions},
};

use entity_bundle::entity_bundle;
use rigidbody_bundle::rigidbody_bundle;

use super::{
    components::{CounterWindow, CounterWindowSensor},
    functions::{lock_closed_action, lock_open_action, toggle_open_action, unlock_action},
};

pub const COUNTER_WINDOW_COLLISION_Y: f32 = 0.5;

pub struct CounterWindowBundle;

impl CounterWindowBundle {
    pub fn spawn(mut spawn_data: SpawnData) -> Entity {
        let entity_name = spawn_data.entity_name;
        let department_name;

        if entity_name == "securityCounterWindow" {
            department_name = "security";
        } else if entity_name == "bridgeCounterWindow" {
            department_name = "bridge";
        } else {
            warn!("Unrecognized counterwindow sub-type {}", entity_name);
            return Entity::from_bits(0);
        }
        let entity = spawn_data.commands.spawn().id();

        let default_transform = Transform::identity();

        let rigidbody_bundle = rigidbody_bundle();
        let entity_bundle = entity_bundle(default_transform, department_name, entity_name);

        if spawn_data.correct_transform {
            spawn_data.entity_transform.rotation = default_transform.rotation;
        }

        rigidbody_builder(
            &mut spawn_data.commands,
            entity,
            RigidBodySpawnData {
                rigidbody_dynamic: false,
                rigid_transform: spawn_data.entity_transform,
                collider: rigidbody_bundle.collider,
                collider_transform: rigidbody_bundle.collider_transform,
                collider_friction: rigidbody_bundle.collider_friction,
                ..Default::default()
            },
        );

        base_entity_builder(
            &mut spawn_data.commands,
            entity,
            BaseEntityData {
                dynamicbody: false,
                entity_type: entity_bundle.entity_name.clone(),
                examinable: entity_bundle.examinable,
                tab_actions_option: Some(TabActions {
                    tab_actions: vec![
                        TabAction {
                            id: "actions::counter_windows/toggleopen".to_string(),
                            text: "Toggle Open".to_string(),
                            tab_list_priority: 100,
                            prerequisite_check: Arc::new(toggle_open_action),
                            belonging_entity: Some(entity),
                        },
                        TabAction {
                            id: "actions::counter_windows/lockopen".to_string(),
                            text: "Lock Open".to_string(),
                            tab_list_priority: 99,
                            prerequisite_check: Arc::new(lock_open_action),
                            belonging_entity: Some(entity),
                        },
                        TabAction {
                            id: "actions::counter_windows/lockclosed".to_string(),
                            text: "Lock Closed".to_string(),
                            tab_list_priority: 98,
                            prerequisite_check: Arc::new(lock_closed_action),
                            belonging_entity: Some(entity),
                        },
                        TabAction {
                            id: "actions::counter_windows/unlock".to_string(),
                            text: "Unlock".to_string(),
                            tab_list_priority: 97,
                            prerequisite_check: Arc::new(unlock_action),
                            belonging_entity: Some(entity),
                        },
                    ],
                }),
                health: Health {
                    is_combat_obstacle: true,
                    is_laser_obstacle: false,
                    is_reach_obstacle: true,
                    ..Default::default()
                },
                entity_group: EntityGroup::AirLock,
                ..Default::default()
            },
        );

        spawn_data.commands.entity(entity).insert(CounterWindow {
            access_permissions: vec![ShipAuthorizationEnum::Security],
            ..Default::default()
        });

        let rigid_body = RigidBody::Fixed;

        let masks = get_bit_masks(ColliderGroup::Standard);

        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Average;

        let sensor = Sensor(true);

        let mut sensor_builder = spawn_data.commands.spawn();
        sensor_builder
            .insert(rigid_body)
            .insert(spawn_data.entity_transform);
        sensor_builder.with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(1., 1., 1.))
                .insert(Transform::from_translation(Vec3::new(0., -1., 0.)))
                .insert(friction)
                .insert(CollisionGroups::new(masks.0, masks.1))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(sensor);
        });

        let child = sensor_builder
            .insert_bundle((
                CounterWindowSensor { parent: entity },
                spawn_data.entity_transform,
                EntityData {
                    entity_class: "child".to_string(),
                    entity_name: "counterWindowSensor".to_string(),
                    entity_group: EntityGroup::CounterWindowSensor,
                },
            ))
            .id();

        spawn_data.commands.entity(entity).push_children(&[child]);

        entity
    }
}
