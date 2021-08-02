use bevy::{prelude::{Commands, Entity, EventReader, Query, ResMut}};
use bevy_rapier3d::{prelude::RigidBodyPosition};

use crate::space_core::{bundles::{counter_window_closed_sfx::{CounterWindowClosedSfxBundle, PLAY_BACK_DURATION as CLOSED_PLAY_BACK_DURATION}, counter_window_denied_sfx::{CounterWindowDeniedSfxBundle, PLAY_BACK_DURATION as DENIED_PLAY_BACK_DURATION}, counter_window_open_sfx::{CounterWindowOpenSfxBundle, PLAY_BACK_DURATION as OPEN_PLAY_BACK_DURATION}}, components::{counter_window::{CounterWindow, CounterWindowAccessLightsStatus, CounterWindowStatus}, counter_window_closed_timer::CounterWindowClosedTimer, counter_window_denied_timer::CounterWindowDeniedTimer, counter_window_open_timer::CounterWindowOpenTimer, counter_window_sensor::CounterWindowSensor, entity_data::EntityGroup, pawn::Pawn, sfx::sfx_auto_destroy, space_access::SpaceAccess, static_transform::StaticTransform}, events::physics::counter_window_sensor_collision::CounterWindowSensorCollision, resources::sfx_auto_destroy_timers::SfxAutoDestroyTimers};

pub fn counter_window_events(
    mut counter_window_sensor_collisions : EventReader<CounterWindowSensorCollision>,
    mut counter_window_query : Query<(
        &mut CounterWindow,
        &mut RigidBodyPosition,
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
                    sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers,CLOSED_PLAY_BACK_DURATION);
                    

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

        match counter_window_components_result {
            Ok(result) => {
                counter_window_component = result.0;
                counter_window_rigid_body_position_component = result.1;
                counter_window_static_transform_component = result.2;
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

        if pawn_has_permission == true {

            if !matches!(counter_window_component.status, CounterWindowStatus::Open) {
                let sfx_entity = commands.spawn().insert_bundle(CounterWindowOpenSfxBundle::new(counter_window_static_transform_component.transform)).id();
                sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers,OPEN_PLAY_BACK_DURATION);
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
            sfx_auto_destroy(sfx_entity,&mut auto_destroy_timers,DENIED_PLAY_BACK_DURATION);

        }


    }


}
