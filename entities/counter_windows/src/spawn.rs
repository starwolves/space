use std::collections::BTreeMap;

pub fn get_default_transform() -> Transform {
    Transform::identity()
}

impl BaseEntitySummonable<NoData> for CounterWindowSummoner {
    fn get_bundle(&self, spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
        let entity_name = spawn_data.entity_name.clone();
        let department_name;

        if entity_name == SECURITY_COUNTER_WINDOW_ENTITY_NAME {
            department_name = "security";
        } else if entity_name == BRIDGE_COUNTER_WINDOW_ENTITY_NAME {
            department_name = "bridge";
        } else {
            warn!("Unrecognized counterwindow sub-type {}", entity_name);
            department_name = "ERR";
        }
        let mut examine_map = BTreeMap::new();

        examine_map.insert(
            0,
            "An airtight ".to_string()
                + department_name
                + " window. It will only grant access to those authorised to use it.",
        );
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );
        BaseEntityBundle {
            entity_name: entity_name,
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: department_name.to_string() + " window",
                    n: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            health: Health {
                is_combat_obstacle: true,
                is_laser_obstacle: false,
                is_reach_obstacle: true,
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
use api::{
    chat::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
    converters::string_transform_to_transform,
    data::NoData,
    entity_updates::{EntityData, EntityGroup},
    examinable::{Examinable, RichName},
    health::Health,
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{warn, Commands, EventReader, EventWriter, GlobalTransform, Transform},
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider, Friction};
use entity::{
    entity_data::RawSpawnEvent,
    spawn::{BaseEntityBundle, BaseEntitySummonable, DefaultSpawnEvent, SpawnData, SpawnEvent},
};
use pawn::pawn::ShipAuthorizationEnum;
use physics::physics::{get_bit_masks, ColliderGroup};
use rigid_body::spawn::{RigidBodyBundle, RigidBodySummonable};

use super::counter_window_events::{CounterWindow, CounterWindowSensor};

impl RigidBodySummonable<NoData> for CounterWindowSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> RigidBodyBundle {
        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Average;

        RigidBodyBundle {
            collider: Collider::cuboid(0.1, 0.5, 1.),
            collider_transform: Transform::from_translation(Vec3::new(
                0.,
                COUNTER_WINDOW_COLLISION_Y,
                0.,
            )),
            collider_friction: friction,
            rigidbody_dynamic: false,
            ..Default::default()
        }
    }
}

use bevy_rapier3d::prelude::{ActiveEvents, CollisionGroups, RigidBody, Sensor};

pub const COUNTER_WINDOW_COLLISION_Y: f32 = 0.5;

pub struct CounterWindowSummoner;

pub fn summon_counter_window<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(CounterWindow {
                access_permissions: vec![ShipAuthorizationEnum::Security],
                ..Default::default()
            });

        let rigid_body = RigidBody::Fixed;

        let masks = get_bit_masks(ColliderGroup::Standard);

        let mut friction = Friction::coefficient(0.);
        friction.combine_rule = CoefficientCombineRule::Average;

        let sensor = Sensor;

        commands
            .entity(spawn_event.spawn_data.entity)
            .with_children(|children| {
                children
                    .spawn()
                    .insert(rigid_body)
                    .insert(GlobalTransform::identity())
                    .insert(Transform::identity())
                    .insert_bundle((
                        CounterWindowSensor {
                            parent: spawn_event.spawn_data.entity,
                        },
                        EntityData {
                            entity_class: "child".to_string(),
                            entity_name: "counterWindowSensor".to_string(),
                            entity_group: EntityGroup::CounterWindowSensor,
                        },
                    ))
                    .with_children(|children| {
                        children
                            .spawn()
                            .insert(Collider::cuboid(1., 1., 1.))
                            .insert(Transform::from_translation(Vec3::new(0., -1., 0.)))
                            .insert(GlobalTransform::default())
                            .insert(friction)
                            .insert(CollisionGroups::new(masks.0, masks.1))
                            .insert(ActiveEvents::COLLISION_EVENTS)
                            .insert(sensor);
                    });
            });
    }
}

pub const SECURITY_COUNTER_WINDOW_ENTITY_NAME: &str = "securityCounterWindow";
pub const BRIDGE_COUNTER_WINDOW_ENTITY_NAME: &str = "bridgeCounterWindow";

pub fn summon_raw_counter_window(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<CounterWindowSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != SECURITY_COUNTER_WINDOW_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != BRIDGE_COUNTER_WINDOW_ENTITY_NAME
        {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: CounterWindowSummoner,
        });
    }
}

pub fn default_summon_counter_window(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<CounterWindowSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != SECURITY_COUNTER_WINDOW_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != BRIDGE_COUNTER_WINDOW_ENTITY_NAME
        {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: CounterWindowSummoner,
        });
    }
}
