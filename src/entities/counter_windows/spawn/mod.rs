pub mod entity_bundle;
pub mod rigidbody_bundle;

use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};
use bevy_hierarchy::BuildChildren;
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{
    ActiveEvents, CoefficientCombineRule, Collider, CollisionGroups, Friction, RigidBody, Sensor,
};
use bevy_transform::prelude::Transform;

use crate::core::{
    entity::{
        components::{EntityData, EntityGroup},
        events::RawSpawnEvent,
        functions::string_to_type_converters::string_transform_to_transform,
        resources::SpawnData,
        spawn::{DefaultSpawnEvent, SpawnEvent},
    },
    pawn::components::ShipAuthorizationEnum,
    physics::functions::{get_bit_masks, ColliderGroup},
};

use super::components::{CounterWindow, CounterWindowSensor};

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

        let sensor = Sensor(true);

        let mut sensor_builder = commands.spawn();
        sensor_builder
            .insert(rigid_body)
            .insert(spawn_event.spawn_data.entity_transform);
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
                CounterWindowSensor {
                    parent: spawn_event.spawn_data.entity,
                },
                spawn_event.spawn_data.entity_transform,
                EntityData {
                    entity_class: "child".to_string(),
                    entity_name: "counterWindowSensor".to_string(),
                    entity_group: EntityGroup::CounterWindowSensor,
                },
            ))
            .id();

        commands
            .entity(spawn_event.spawn_data.entity)
            .push_children(&[child]);
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
