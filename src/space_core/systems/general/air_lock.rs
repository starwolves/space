

use bevy::{prelude::{Commands, Entity, EventReader, Query, ResMut}};
use bevy_rapier3d::{prelude::RigidBodyPosition};

use crate::space_core::{bundles::{air_lock_closed_sfx::{AirLockClosedSfxBundle}, air_lock_denied_sfx::{AirLockDeniedSfxBundle}, air_lock_open_sfx::{AirLockOpenSfxBundle}}, components::{air_lock::{AccessLightsStatus, AirLock, AirLockStatus}, air_lock_closed_timer::AirLockClosedTimer, air_lock_denied_timer::AirLockDeniedTimer, air_lock_open_timer::AirLockOpenTimer, entity_data::{EntityGroup}, pawn::Pawn, sfx::sfx_auto_destroy, space_access::SpaceAccess, static_transform::StaticTransform}, events::physics::air_lock_collision::AirLockCollision, resources::sfx_auto_destroy_timers::SfxAutoDestroyTimers};

pub fn air_lock_events(
    mut air_lock_collisions : EventReader<AirLockCollision>,
    mut air_lock_query : Query<(
        &mut AirLock,
        &mut RigidBodyPosition,
        &StaticTransform,
        Option<&mut AirLockOpenTimer>,
        Option<&mut AirLockDeniedTimer>,
        Option<&mut AirLockClosedTimer>,
        Entity
    )>,
    pawn_query : Query<(&Pawn, &SpaceAccess)>,
    mut auto_destroy_timers : ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands
) {

    for (
        mut air_lock_component,
        mut rigid_body_position_component,
        static_transform_component,
        timer_open_component_option,
        timer_denied_component_option,
        timer_closed_component_option,
        air_lock_entity
    ) in air_lock_query.iter_mut() {

        match timer_open_component_option {
            Some(mut timer_component) => {

                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    air_lock_component.status = AirLockStatus::Closed;                    

                    commands.entity(air_lock_entity).insert(AirLockClosedTimer::default());

                }

            }
            None => {}
        }

        match timer_closed_component_option {
            Some(mut timer_component) => {

                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    let mut air_lock_rigid_body_position = rigid_body_position_component.position;

                    air_lock_rigid_body_position.translation.y -= 2.;

                    rigid_body_position_component.position = air_lock_rigid_body_position;

                    air_lock_component.access_lights = AccessLightsStatus::Neutral;

                    let sfx_entity = commands.spawn().insert_bundle(AirLockClosedSfxBundle::new(static_transform_component.transform)).id();
                    sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers);
                    

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
        let mut air_lock_rigid_body_position_component;
        let air_lock_static_transform_component;

        match air_lock_components_result {
            Ok(result) => {
                air_lock_component = result.0;
                air_lock_rigid_body_position_component = result.1;
                air_lock_static_transform_component = result.2;
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


            let mut air_lock_rigid_body_position = air_lock_rigid_body_position_component.position;

            air_lock_rigid_body_position.translation.y += 2.;

            air_lock_rigid_body_position_component.position = air_lock_rigid_body_position;

            commands.entity(air_lock_entity).insert(AirLockOpenTimer::default());

            let sfx_entity = commands.spawn().insert_bundle(AirLockOpenSfxBundle::new(air_lock_static_transform_component.transform)).id();
            sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers);

        } else {
            air_lock_component.access_lights = AccessLightsStatus::Denied;

            commands.entity(air_lock_entity).insert(AirLockDeniedTimer::default());

            let sfx_entity = commands.spawn().insert_bundle(AirLockDeniedSfxBundle::new(air_lock_static_transform_component.transform)).id();
            sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers);

        }


    }

}
