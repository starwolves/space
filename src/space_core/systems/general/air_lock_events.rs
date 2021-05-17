use bevy::{core::Timer, prelude::{Commands, EventReader, Query, ResMut}};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::space_core::{components::{air_lock::{AccessLightsStatus, AirLock, AirLockStatus}, air_lock_denied_timer::AirLockDeniedTimer, air_lock_open_timer::AirLockOpenTimer, entity_data::EntityGroup, pawn::Pawn, space_access::SpaceAccess}, events::physics::air_lock_collision::AirLockCollision};

pub fn air_lock_events(
    mut rigid_bodies: ResMut<RigidBodySet>,
    mut air_lock_collisions : EventReader<AirLockCollision>,
    mut air_lock_query : Query<(
        &mut AirLock,
        &RigidBodyHandleComponent,
        Option<&mut AirLockOpenTimer>,
        Option<&mut AirLockDeniedTimer>
    )>,
    pawn_query : Query<(&Pawn, &SpaceAccess)>,
    mut commands: Commands
) {

    for (
        mut air_lock_component,
        rigid_body_handle_component,
        timer_open_component_option,
        timer_denied_component_option
    ) in air_lock_query.iter_mut() {

        match timer_open_component_option {
            Some(mut timer_component) => {

                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    air_lock_component.status = AirLockStatus::Closed;
                    

                    let air_lock_rigid_body = rigid_bodies.get_mut(rigid_body_handle_component.handle())
                    .expect("air_lock_events.rs rigidbody handle was not present in RigidBodySet resource. (0)");

                    let mut air_lock_rigid_body_position = *air_lock_rigid_body.position();

                    air_lock_rigid_body_position.translation.y -= 2.;

                    air_lock_rigid_body.set_position(air_lock_rigid_body_position, true);

                }

            }
            None => {}
        }

        match timer_denied_component_option {
            Some(mut timer_component) => {

                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    air_lock_component.access_lights = AccessLightsStatus::Neutral;

                }

            }
            None => {}
        }

        

    }
    
    for collision_event in air_lock_collisions.iter() {

        

        if collision_event.started == false {
            continue;
        }

        

        let air_lock_entity;
        let pawn_entity;

        if matches!(collision_event.collider1_group, EntityGroup::AirLock) {

            air_lock_entity = collision_event.collider1_entity;
            pawn_entity = collision_event.collider2_entity;

        } else {

            air_lock_entity = collision_event.collider2_entity;
            pawn_entity = collision_event.collider1_entity;

        }

        
        let pawn_space_access_component_result = pawn_query.get_component::<SpaceAccess>(pawn_entity);
        let pawn_space_access_component;

        

        match pawn_space_access_component_result {
            Ok(result) => {
                pawn_space_access_component = result;
            }
            Err(_err) => {continue;}
        }

        let air_lock_components_result = air_lock_query.get_mut(air_lock_entity);

        let mut air_lock_component;
        let air_lock_rigid_body_handle_component;

        match air_lock_components_result {
            Ok(result) => {
                air_lock_component = result.0;
                air_lock_rigid_body_handle_component = result.1;
            }
            Err(_err) => {continue;}
        }

        let mut pawn_has_permission = false;

        for space_permission in &air_lock_component.access_permissions {
            
            if pawn_space_access_component.access.contains(space_permission) == true {
                pawn_has_permission=true;
                break;
            }

        }

        if pawn_has_permission == true {


            air_lock_component.status = AirLockStatus::Open;
            air_lock_component.access_lights = AccessLightsStatus::Granted;

            let air_lock_rigid_body = rigid_bodies.get_mut(air_lock_rigid_body_handle_component.handle())
            .expect("air_lock_events.rs rigidbody handle was not present in RigidBodySet resource. (1)");

            let mut air_lock_rigid_body_position = *air_lock_rigid_body.position();

            air_lock_rigid_body_position.translation.y += 2.;

            air_lock_rigid_body.set_position(air_lock_rigid_body_position, true);

            commands.entity(air_lock_entity).insert(AirLockOpenTimer {
                timer : Timer::from_seconds(5.0, false)
            });

        } else {
            air_lock_component.access_lights = AccessLightsStatus::Denied;

            commands.entity(air_lock_entity).insert(AirLockDeniedTimer {
                timer : Timer::from_seconds(5.0, false)
            });

        }


    }

}
