use bevy::{prelude::{Commands, Entity, EventReader, Query, ResMut, Res}, core::{Time, Timer}};
use bevy_rapier3d::{prelude::{RigidBodyPositionComponent}};

use crate::space::{entities::{counter_window_security::{components::{CounterWindow, CounterWindowOpenTimer, CounterWindowDeniedTimer, CounterWindowClosedTimer, CounterWindowSensor, CounterWindowStatus, CounterWindowAccessLightsStatus}, events::CounterWindowSensorCollision}, sfx::counter_window::{counter_window_closed_sfx::CounterWindowClosedSfxBundle, counter_window_open_sfx::CounterWindowOpenSfxBundle, counter_window_denied_sfx::CounterWindowDeniedSfxBundle}}, core::{pawn::components::{Pawn, SpaceAccess}, sfx::{components::sfx_auto_destroy, resources::SfxAutoDestroyTimers}, static_body::components::StaticTransform, entity::components::EntityGroup}};

pub fn counter_window_events(
    mut counter_window_sensor_collisions : EventReader<CounterWindowSensorCollision>,
    mut counter_window_query : Query<(
        &mut CounterWindow,
        &mut RigidBodyPositionComponent,
        &StaticTransform,
        Option<&mut CounterWindowOpenTimer>,
        Option<&mut CounterWindowDeniedTimer>,
        Option<&mut CounterWindowClosedTimer>,
        Entity
    )>,
    counter_window_sensor_query : Query<&CounterWindowSensor>,
    pawn_query : Query<(&Pawn, &SpaceAccess)>,
    mut auto_destroy_timers : ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands
) {

    
    for (
        mut counter_window_component,
        mut rigid_body_position_component,
        static_transform_component,
        counter_window_open_timer_option,
        counter_window_denied_timer_option,
        counter_window_closed_timer_option,
        counter_window_entity
    ) in counter_window_query.iter_mut() {

        match counter_window_open_timer_option {
            Some(mut timer_component) => {

                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    counter_window_component.status = CounterWindowStatus::Closed;

                    commands.entity(counter_window_entity).insert(CounterWindowClosedTimer::default());

                }

            }
            None => {}
        }

        match counter_window_closed_timer_option {
            Some(mut timer_component) => {

                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();


                    let mut counter_window_rigid_body_position = rigid_body_position_component.position;

                    counter_window_rigid_body_position.translation.y -= 2.;

                    rigid_body_position_component.position = counter_window_rigid_body_position;

                    counter_window_component.access_lights = CounterWindowAccessLightsStatus::Neutral;

                    let sfx_entity = commands.spawn().insert_bundle(CounterWindowClosedSfxBundle::new(static_transform_component.transform)).id();
                    sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers);
                    
                }

            }
            None => {}
        }

        match counter_window_denied_timer_option {
            Some(mut timer_component) => {

                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    counter_window_component.access_lights = CounterWindowAccessLightsStatus::Neutral;
                }

            }
            None => {}
        }

    }

    for collision_event in counter_window_sensor_collisions.iter() {

        if collision_event.started == false {
            continue;
        }

        let counter_window_sensor_entity;
        let pawn_entity;

        if matches!(collision_event.collider1_group, EntityGroup::CounterWindowSensor) {

            counter_window_sensor_entity = collision_event.collider1_entity;
            pawn_entity = collision_event.collider2_entity;

        } else {

            counter_window_sensor_entity = collision_event.collider2_entity;
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


        let counter_window_sensor_components_result = counter_window_sensor_query.get_component::<CounterWindowSensor>(counter_window_sensor_entity);
        let counter_window_sensor_component;

        match counter_window_sensor_components_result {
            Ok(counter_window_sensor) => {
                counter_window_sensor_component = counter_window_sensor;
            }
            Err(_err) => {continue;}
        }


        let counter_window_components_result = counter_window_query.get_mut(counter_window_sensor_component.parent);

        let mut counter_window_component;
        let mut counter_window_rigid_body_position_component;
        let counter_window_static_transform_component;
        let counter_window_closed_timer_option;

        match counter_window_components_result {
            Ok(result) => {
                counter_window_component = result.0;
                counter_window_rigid_body_position_component = result.1;
                counter_window_static_transform_component = result.2;
                counter_window_closed_timer_option = result.5;
            }
            Err(_err) => {continue;}
        }

        let mut pawn_has_permission = false;

        for space_permission in &counter_window_component.access_permissions {
            
            if pawn_space_access_component.access.contains(space_permission) == true {
                pawn_has_permission=true;
                break;
            }

        }
        
        match counter_window_closed_timer_option {
            Some(mut counter_window_closed_timer) => {
                counter_window_closed_timer.timer.pause();
                counter_window_closed_timer.timer.reset();
            },
            None => {},
        }

        if pawn_has_permission == true {

            if !matches!(counter_window_component.status, CounterWindowStatus::Open) {
                let sfx_entity = commands.spawn().insert_bundle(CounterWindowOpenSfxBundle::new(counter_window_static_transform_component.transform)).id();
                sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers);
            }

            counter_window_component.status = CounterWindowStatus::Open;
            counter_window_component.access_lights = CounterWindowAccessLightsStatus::Granted;


            let mut counter_window_rigid_body_position = counter_window_rigid_body_position_component.position;

            counter_window_rigid_body_position.translation.y += 2.;

            counter_window_rigid_body_position_component.position = counter_window_rigid_body_position;

            commands.entity(counter_window_sensor_component.parent).insert(CounterWindowOpenTimer::default());

        } else {

            counter_window_component.access_lights = CounterWindowAccessLightsStatus::Denied;

            commands.entity(counter_window_sensor_component.parent).insert(CounterWindowDeniedTimer::default());

            let sfx_entity = commands.spawn().insert_bundle(CounterWindowDeniedSfxBundle::new(counter_window_static_transform_component.transform)).id();
            sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers);

        }


    }


}



pub fn counter_window_tick_timers(
    time: Res<Time>, 
    mut query_timer: Query<&mut Timer>,
    mut query_counter_window_open_timer: Query<&mut CounterWindowOpenTimer>,
    mut query_counter_window_denied_timer: Query<&mut CounterWindowDeniedTimer>,
    mut query_counter_window_closed_timer: Query<&mut CounterWindowClosedTimer>,
    
    
    mut sfx_auto_destroy_timers : ResMut<SfxAutoDestroyTimers>,
    mut commands : Commands
){
    for mut timer in query_timer.iter_mut() {
        timer.tick(time.delta());
    }
    for mut timer in query_counter_window_open_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_counter_window_denied_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_counter_window_closed_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }

    let mut expired_sfx_entities : Vec<Entity> =  vec![];

    for (sfx_entity, incremental) in &mut sfx_auto_destroy_timers.timers {

        *incremental+=1;
        if incremental >= &mut 2 {
            expired_sfx_entities.push(*sfx_entity);
        }

    }

    for i in 0..expired_sfx_entities.len() {

        let this_entity_id = expired_sfx_entities[i];

        let mut j = 0;
        for (sfx_entity, _timer) in &mut sfx_auto_destroy_timers.timers {
            if this_entity_id == *sfx_entity {
                break;
            }
            j+=1;
        }

        sfx_auto_destroy_timers.timers.remove(j);

        commands.entity(this_entity_id).despawn();

    }
    

}