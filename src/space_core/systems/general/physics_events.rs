use bevy::prelude::{EventWriter, Query, Res};
use bevy_rapier3d::physics::{EventQueue, ColliderHandleComponent };

use crate::space_core::{components::entity_data::{EntityData, EntityGroup}, events::physics::air_lock_collision::AirLockCollision};

pub fn physics_events(
    physics_events: Res<EventQueue>,
    interesting_entities_query : Query<(&ColliderHandleComponent, &EntityData)>,
    mut air_lock_collision_event : EventWriter<AirLockCollision>
) {

    while let Ok(intersection_event) = physics_events.intersection_events.pop() {

        // Search for entities in query that is collider1 and collider2.
        // If none or only 1 is found in the query it is a uninteresting collision.
        // Check if one collider is of Pawn type and other is of AirLock type.

        let mut first_collider_interesting = false;
        let mut second_collider_interesting = false;


        let mut first_collider_group = EntityGroup::None;
        let mut second_collider_group = EntityGroup::None;

        for (
            collider_handle_component,
            entity_data_component
        ) in interesting_entities_query.iter() {

            let interesting_entity_collider_handle = collider_handle_component.handle();

            if first_collider_interesting == false && interesting_entity_collider_handle == intersection_event.collider1 {

                first_collider_group = entity_data_component.entity_group;
                first_collider_interesting = true;

            } else if second_collider_interesting == false && interesting_entity_collider_handle == intersection_event.collider2 {

                second_collider_group = entity_data_component.entity_group;
                second_collider_interesting = true;

            }

            if first_collider_interesting == true && second_collider_interesting == true {
                break;
            }

        }

        if first_collider_interesting == false || second_collider_interesting == false {
            continue;
        }

        if matches!(first_collider_group, EntityGroup::AirLock) || matches!(second_collider_group, EntityGroup::AirLock) {

            air_lock_collision_event.send(AirLockCollision {

                collider1_handle : intersection_event.collider1,
                collider2_handle : intersection_event.collider2,

                collider1_group : first_collider_group,
                collider2_group: second_collider_group,

                intersecting : intersection_event.intersecting

            });

        }



    }

    while let Ok(_contact_event) = physics_events.contact_events.pop() {

    }

}
