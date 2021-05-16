use bevy::prelude::{Entity, EventWriter, Query, Res};
use bevy_rapier3d::physics::{EventQueue, ColliderHandleComponent };

use crate::space_core::{components::entity_data::{EntityData, EntityGroup}, events::physics::air_lock_collision::AirLockCollision};

pub fn physics_events(
    physics_events: Res<EventQueue>,
    interesting_entities_query : Query<(
        Entity,
        &ColliderHandleComponent,
        &EntityData
    )>,
    mut air_lock_collision_event : EventWriter<AirLockCollision>
) {

    while let Ok(_intersection_event) = physics_events.intersection_events.pop() {
        // This fires with sensor collider types.
    }

    while let Ok(contact_event) = physics_events.contact_events.pop() {

        let mut collision_started = false;
        let collider1_handle;
        let collider2_handle;

        match contact_event {
            bevy_rapier3d::rapier::geometry::ContactEvent::Started(collider1, collider2) => {
                collision_started=true;
                collider1_handle = collider1;
                collider2_handle = collider2;
            }
            bevy_rapier3d::rapier::geometry::ContactEvent::Stopped(collider1, collider2) => {
                collider1_handle = collider1;
                collider2_handle = collider2;
            }
        }

        let mut first_collider_interesting = false;
        let mut second_collider_interesting = false;


        let mut first_collider_group = EntityGroup::None;
        let mut second_collider_group = EntityGroup::None;

        let mut collider1_entity = None;
        let mut collider2_entity = None;

        for (
            entity,
            collider_handle_component,
            entity_data_component
        ) in interesting_entities_query.iter() {

            let interesting_entity_collider_handle = collider_handle_component.handle();

            if first_collider_interesting == false && interesting_entity_collider_handle == collider1_handle && matches!(entity_data_component.entity_group, EntityGroup::None) == false {

                first_collider_group = entity_data_component.entity_group;
                first_collider_interesting = true;
                collider1_entity = Some(entity);

            } else if second_collider_interesting == false && interesting_entity_collider_handle == collider2_handle && matches!(entity_data_component.entity_group, EntityGroup::None) == false  {

                second_collider_group = entity_data_component.entity_group;
                second_collider_interesting = true;
                collider2_entity = Some(entity);

            }

            if first_collider_interesting == true && second_collider_interesting == true {
                break;
            }

        }

        if matches!(first_collider_group, EntityGroup::AirLock) || matches!(second_collider_group, EntityGroup::AirLock) {


            air_lock_collision_event.send(AirLockCollision {

                collider1_entity : collider1_entity.unwrap(),
                collider2_entity : collider2_entity.unwrap(),

                collider1_group : first_collider_group,
                collider2_group : second_collider_group,

                started : collision_started

            });

            

        }

    }

}
