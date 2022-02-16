use bevy::prelude::{Entity, EventReader, EventWriter, Query};
use bevy_rapier3d::{
    prelude::{ContactEvent, IntersectionEvent, IntoEntity, RigidBodyPositionComponent},
    rapier::geometry::ColliderHandle,
};

use crate::space::{
    core::entity::components::{EntityData, EntityGroup},
    entities::{
        air_lock_security::events::AirLockCollision,
        counter_window_security::events::CounterWindowSensorCollision,
    },
};

pub fn physics_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
    interesting_entities_query: Query<(Entity, &EntityData, &RigidBodyPositionComponent)>,
    mut air_lock_collision_event: EventWriter<AirLockCollision>,
    mut counter_window_collision_event: EventWriter<CounterWindowSensorCollision>,
) {
    for intersection_event in intersection_events.iter() {
        // This fires with sensor collider types.
        let collider1_handle = intersection_event.collider1;
        let collider2_handle = intersection_event.collider2;
        let collision_started = intersection_event.intersecting;

        process_physics_event(
            collider1_handle,
            collider2_handle,
            collision_started,
            &interesting_entities_query,
            &mut air_lock_collision_event,
            &mut counter_window_collision_event,
        );
    }

    for contact_event in contact_events.iter() {
        let mut collision_started = false;
        let collider1_handle;
        let collider2_handle;

        match contact_event {
            bevy_rapier3d::rapier::geometry::ContactEvent::Started(collider1, collider2) => {
                collision_started = true;
                collider1_handle = collider1;
                collider2_handle = collider2;
            }
            bevy_rapier3d::rapier::geometry::ContactEvent::Stopped(collider1, collider2) => {
                collider1_handle = collider1;
                collider2_handle = collider2;
            }
        }

        process_physics_event(
            *collider1_handle,
            *collider2_handle,
            collision_started,
            &interesting_entities_query,
            &mut air_lock_collision_event,
            &mut counter_window_collision_event,
        );
    }
}

fn process_physics_event(
    collider1_handle: ColliderHandle,
    collider2_handle: ColliderHandle,
    collision_started: bool,
    interesting_entities_query: &Query<(Entity, &EntityData, &RigidBodyPositionComponent)>,
    air_lock_collision_event: &mut EventWriter<AirLockCollision>,
    counter_window_collision_event: &mut EventWriter<CounterWindowSensorCollision>,
) {
    let mut first_collider_group = EntityGroup::None;
    let mut second_collider_group = EntityGroup::None;

    let collider1_entity = collider1_handle.entity();
    let collider2_entity = collider2_handle.entity();

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
