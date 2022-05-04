use bevy_app::{App, Plugin};
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    schedule::SystemSet,
    system::Query,
};
use bevy_rapier3d::pipeline::CollisionEvent;
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        entity::components::{EntityData, EntityGroup},
        plugin::PostUpdateLabels,
    },
    entities::{
        air_locks::events::AirLockCollision, counter_windows::events::CounterWindowSensorCollision,
    },
};

pub fn physics_events(
    mut collision_events: EventReader<CollisionEvent>,
    interesting_entities_query: Query<(Entity, &EntityData, &Transform)>,
    mut air_lock_collision_event: EventWriter<AirLockCollision>,
    mut counter_window_collision_event: EventWriter<CounterWindowSensorCollision>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(collider1_handle, collider2_handle, _flags) => {
                process_physics_event(
                    *collider1_handle,
                    *collider2_handle,
                    true,
                    &interesting_entities_query,
                    &mut air_lock_collision_event,
                    &mut counter_window_collision_event,
                );
            }
            CollisionEvent::Stopped(collider1_handle, collider2_handle, _flags) => {
                process_physics_event(
                    *collider1_handle,
                    *collider2_handle,
                    false,
                    &interesting_entities_query,
                    &mut air_lock_collision_event,
                    &mut counter_window_collision_event,
                );
            }
        }
    }
}

fn process_physics_event(
    collider1_entity: Entity,
    collider2_entity: Entity,
    collision_started: bool,
    interesting_entities_query: &Query<(Entity, &EntityData, &Transform)>,
    air_lock_collision_event: &mut EventWriter<AirLockCollision>,
    counter_window_collision_event: &mut EventWriter<CounterWindowSensorCollision>,
) {
    let mut first_collider_group = EntityGroup::None;
    let mut second_collider_group = EntityGroup::None;

    let collider1_components = interesting_entities_query.get(collider1_entity).unwrap();
    let collider2_components = interesting_entities_query.get(collider2_entity).unwrap();

    if matches!(collider1_components.1.entity_group, EntityGroup::None) == false {
        first_collider_group = collider1_components.1.entity_group;
    }

    if matches!(collider2_components.1.entity_group, EntityGroup::None) == false {
        second_collider_group = collider2_components.1.entity_group;
    }

    if matches!(first_collider_group, EntityGroup::AirLock)
        || matches!(second_collider_group, EntityGroup::AirLock)
    {
        air_lock_collision_event.send(AirLockCollision {
            collider1_entity: collider1_entity,
            collider2_entity: collider2_entity,

            collider1_group: first_collider_group,
            collider2_group: second_collider_group,

            started: collision_started,
        });
    } else if (matches!(first_collider_group, EntityGroup::CounterWindowSensor)
        || matches!(second_collider_group, EntityGroup::CounterWindowSensor))
    {
        counter_window_collision_event.send(CounterWindowSensorCollision {
            collider1_entity: collider1_entity,
            collider2_entity: collider2_entity,

            collider1_group: first_collider_group,
            collider2_group: second_collider_group,

            started: collision_started,
        });
    }
}

pub struct PhysicsPlugin;

use bevy_app::CoreStage::PostUpdate;

use super::entity_update::world_mode_update;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(physics_events).add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(world_mode_update),
        );
    }
}
