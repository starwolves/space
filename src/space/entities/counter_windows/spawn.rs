use std::collections::{BTreeMap, HashMap};

use bevy_ecs::{entity::Entity, system::Commands};
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{
    ActiveEvents, CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial,
    ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyType,
};
use bevy_transform::{components::Transform, hierarchy::BuildChildren};

use crate::space::{
    core::{
        chat::functions::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        entity::{
            components::{DefaultMapEntity, EntityData, EntityGroup, EntityUpdates},
            functions::transform_to_isometry::transform_to_isometry,
            resources::{SpawnHeldData, SpawnPawnData},
        },
        examinable::components::{Examinable, RichName},
        health::components::Health,
        networking::resources::ConsoleCommandVariantValues,
        pawn::components::SpaceAccessEnum,
        physics::functions::{get_bit_masks, ColliderGroup},
        sensable::components::Sensable,
        static_body::components::StaticTransform,
    },
    entities::counter_windows::components::{CounterWindow, CounterWindowSensor},
};

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

        let window_rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: transform_to_isometry(entity_transform).into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        let window_collider_component = ColliderBundle {
            shape: ColliderShape::cuboid(0.1, 0.593, 1.).into(),
            position: Vec3::new(0., 0., 1.).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                ..Default::default()
            }
            .into(),
            material: ColliderMaterial {
                friction: 0.,
                friction_combine_rule: CoefficientCombineRule::Average,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };

        let sensor_rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: transform_to_isometry(entity_transform).into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        let sensor_collider_component = ColliderBundle {
            collider_type: ColliderType::Sensor.into(),
            shape: ColliderShape::cuboid(1., 1., 1.).into(),
            position: Vec3::new(0., -1., 1.).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                active_events: (ActiveEvents::INTERSECTION_EVENTS),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };

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

        let mut parent_builder = commands.spawn_bundle(window_rigid_body_component);
        let parent = parent_builder
            .insert_bundle(window_collider_component)
            .insert_bundle((
                static_transform_component,
                Sensable::default(),
                CounterWindow {
                    access_permissions: vec![SpaceAccessEnum::Security],
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
            ))
            .id();

        if default_map_spawn {
            parent_builder.insert(DefaultMapEntity);
        }

        let child = commands
            .spawn_bundle(sensor_rigid_body_component)
            .insert_bundle(sensor_collider_component)
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
