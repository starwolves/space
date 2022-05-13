use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::Children;
use bevy_log::{info, warn};
use bevy_transform::prelude::Transform;

use crate::{
    core::{
        atmospherics::{functions::get_atmos_index, resources::AtmosphericsResource},
        chat::functions::{FURTHER_ITALIC_FONT, WARNING_COLOR},
        entity::components::EntityGroup,
        examinable::components::Examinable,
        gridmap::{functions::gridmap_functions::world_to_cell_id, resources::Vec2Int},
        networking::resources::ReliableServerMessage,
        pawn::components::{Pawn, ShipAuthorization},
        sfx::{components::sfx_auto_destroy, resources::SfxAutoDestroyTimers},
        static_body::components::StaticTransform,
    },
    entities::{
        air_locks::{components::LockedStatus, systems::air_lock_events::AirLockCloseRequest},
        counter_windows::{
            components::{
                CounterWindow, CounterWindowAccessLightsStatus, CounterWindowClosedTimer,
                CounterWindowDeniedTimer, CounterWindowOpenTimer, CounterWindowSensor,
                CounterWindowStatus,
            },
            events::{
                CounterWindowLockClosed, CounterWindowLockOpen, CounterWindowSensorCollision,
                CounterWindowUnlock, InputCounterWindowToggleOpen, NetCounterWindow,
            },
        },
        sfx::counter_window::{
            counter_window_closed_sfx::CounterWindowClosedSfxBundle,
            counter_window_denied_sfx::CounterWindowDeniedSfxBundle,
            counter_window_open_sfx::CounterWindowOpenSfxBundle,
        },
    },
};

pub struct CounterWindowOpenRequest {
    pub opener_option: Option<Entity>,
    pub opened: Entity,
}

pub fn counter_window_events(
    mut counter_window_sensor_collisions: EventReader<CounterWindowSensorCollision>,
    mut counter_window_toggle_open_action: EventReader<InputCounterWindowToggleOpen>,
    mut counter_window_query: Query<(
        &mut CounterWindow,
        &mut Transform,
        &StaticTransform,
        Option<&mut CounterWindowOpenTimer>,
        Option<&mut CounterWindowDeniedTimer>,
        Option<&mut CounterWindowClosedTimer>,
        Entity,
        &Children,
        &mut Examinable,
    )>,
    counter_window_sensor_query: Query<&CounterWindowSensor>,
    pawn_query: Query<(&Pawn, &ShipAuthorization)>,
    mut auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
    mut counter_window_lock_open_events: EventReader<CounterWindowLockOpen>,
    mut counter_window_lock_close_events: EventReader<CounterWindowLockClosed>,
    mut unlock_events: EventReader<CounterWindowUnlock>,
    mut net_counterwindows: EventWriter<NetCounterWindow>,
) {
    let mut close_requests = vec![];
    let mut open_requests = vec![];

    for event in unlock_events.iter() {
        match counter_window_query.get_mut(event.locked) {
            Ok((
                mut counter_window_component,
                _rigid_body_position_component,
                _static_transform_component,
                _counter_window_open_timer_option,
                _counter_window_denied_timer_option,
                _counter_window_closed_timer_option,
                _counter_window_entity,
                _children_component,
                mut examinable_component,
            )) => {
                counter_window_component.locked_status = LockedStatus::None;
                counter_window_component.access_lights = CounterWindowAccessLightsStatus::Neutral;

                match counter_window_component.status {
                    CounterWindowStatus::Open => {
                        close_requests.push(AirLockCloseRequest {
                            interacter_option: None,
                            interacted: event.locked,
                        });
                    }
                    CounterWindowStatus::Closed => {}
                }

                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've unlocked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_counterwindows.send(NetCounterWindow {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }

                examinable_component.assigned_texts.remove(&11);
            }
            Err(_rr) => {}
        }
    }

    for event in counter_window_lock_open_events.iter() {
        match counter_window_query.get_mut(event.locked) {
            Ok((
                mut counter_window_component,
                _rigid_body_position_component,
                _static_transform_component,
                _counter_window_open_timer_option,
                _counter_window_denied_timer_option,
                _counter_window_closed_timer_option,
                _counter_window_entity,
                _children_component,
                mut examinable_component,
            )) => {
                counter_window_component.locked_status = LockedStatus::Open;
                match counter_window_component.status {
                    CounterWindowStatus::Open => {}
                    CounterWindowStatus::Closed => {
                        open_requests.push(CounterWindowOpenRequest {
                            opener_option: None,
                            opened: event.locked,
                        });
                    }
                }
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've opened and locked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_counterwindows.send(NetCounterWindow {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }

                examinable_component.assigned_texts.insert(
                    11,
                    "[font=".to_string()
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + WARNING_COLOR
                        + "]It is locked open.[/color][/font]",
                );
            }
            Err(_rr) => {}
        }
    }
    for event in counter_window_lock_close_events.iter() {
        match counter_window_query.get_mut(event.locked) {
            Ok((
                mut counter_window_component,
                _rigid_body_position_component,
                _static_transform_component,
                _counter_window_open_timer_option,
                _counter_window_denied_timer_option,
                _counter_window_closed_timer_option,
                _counter_window_entity,
                _children_component,
                mut examinable_component,
            )) => {
                counter_window_component.locked_status = LockedStatus::Closed;
                match counter_window_component.status {
                    CounterWindowStatus::Open => {
                        close_requests.push(AirLockCloseRequest {
                            interacter_option: None,
                            interacted: event.locked,
                        });
                    }
                    CounterWindowStatus::Closed => {}
                }
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "You've closed and locked the "
                    + &examinable_component.name.get_name()
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_counterwindows.send(NetCounterWindow {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }

                examinable_component.assigned_texts.insert(
                    11,
                    "[font=".to_string()
                        + FURTHER_ITALIC_FONT
                        + "][color="
                        + WARNING_COLOR
                        + "]It is locked shut.[/color][/font]",
                );
            }
            Err(_rr) => {}
        }
    }

    for (
        mut counter_window_component,
        mut rigid_body_position_component,
        static_transform_component,
        counter_window_open_timer_option,
        counter_window_denied_timer_option,
        counter_window_closed_timer_option,
        counter_window_entity,
        _children_component,
        _examinable_component,
    ) in counter_window_query.iter_mut()
    {
        match counter_window_component.locked_status {
            LockedStatus::Open => {
                if !matches!(
                    counter_window_component.access_lights,
                    CounterWindowAccessLightsStatus::Denied
                ) {
                    counter_window_component.access_lights =
                        CounterWindowAccessLightsStatus::Denied;
                }
            }
            LockedStatus::Closed => {
                if !matches!(
                    counter_window_component.access_lights,
                    CounterWindowAccessLightsStatus::Denied
                ) {
                    counter_window_component.access_lights =
                        CounterWindowAccessLightsStatus::Denied;
                }
            }
            LockedStatus::None => {}
        }

        match counter_window_open_timer_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    close_requests.push(AirLockCloseRequest {
                        interacter_option: None,
                        interacted: counter_window_entity,
                    });
                }
            }
            None => {}
        }

        match counter_window_closed_timer_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    let mut counter_window_rigid_body_position =
                        rigid_body_position_component.clone();

                    counter_window_rigid_body_position.translation.y = 0.943;

                    rigid_body_position_component.translation =
                        counter_window_rigid_body_position.translation;
                    rigid_body_position_component.rotation =
                        counter_window_rigid_body_position.rotation;
                    rigid_body_position_component.scale = counter_window_rigid_body_position.scale;

                    counter_window_component.access_lights =
                        CounterWindowAccessLightsStatus::Neutral;

                    let sfx_entity = commands
                        .spawn()
                        .insert_bundle(CounterWindowClosedSfxBundle::new(
                            static_transform_component.transform,
                        ))
                        .id();
                    sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
                }
            }
            None => {}
        }

        match counter_window_denied_timer_option {
            Some(mut timer_component) => {
                if timer_component.timer.finished() == true {
                    timer_component.timer.pause();
                    timer_component.timer.reset();

                    counter_window_component.access_lights =
                        CounterWindowAccessLightsStatus::Neutral;
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

        if matches!(
            collision_event.collider1_group,
            EntityGroup::CounterWindowSensor
        ) {
            counter_window_sensor_entity = collision_event.collider1_entity;
            pawn_entity = collision_event.collider2_entity;
        } else {
            counter_window_sensor_entity = collision_event.collider2_entity;
            pawn_entity = collision_event.collider1_entity;
        }

        let counter_window_entity;

        match counter_window_sensor_query.get(counter_window_sensor_entity) {
            Ok(counter_window_sensor_component) => {
                counter_window_entity = counter_window_sensor_component.parent;
            }
            Err(_rr) => {
                warn!("Couldn't find parent entity of counter window sensor.");
                continue;
            }
        }

        open_requests.push(CounterWindowOpenRequest {
            opener_option: Some(pawn_entity),
            opened: counter_window_entity,
        });
    }

    for event in counter_window_toggle_open_action.iter() {
        let opened_entity = Entity::from_bits(event.opened);

        info!("Toggle open action from {:?}", opened_entity);

        match counter_window_query.get(opened_entity) {
            Ok((
                counter_window_component,
                _rigid_body_position_component,
                _static_transform_component,
                _counter_window_open_timer_option,
                _counter_window_denied_timer_option,
                _counter_window_closed_timer_option,
                _counter_window_entity,
                _children_component,
                _examinable_component,
            )) => {
                match counter_window_component.status {
                    CounterWindowStatus::Open => {
                        close_requests.push(AirLockCloseRequest {
                            interacter_option: Some(event.opener),
                            interacted: opened_entity,
                        });
                    }
                    CounterWindowStatus::Closed => {
                        open_requests.push(CounterWindowOpenRequest {
                            opener_option: Some(event.opener),
                            opened: opened_entity,
                        });
                    }
                }
                break;
                //Should only fire once anyways.
            }
            Err(_rr) => {
                warn!("Couldn't find children component of counter window.");
            }
        };
    }

    for request in open_requests {
        let counter_window_components_result = counter_window_query.get_mut(request.opened);

        let mut counter_window_component;
        let mut counter_window_rigid_body_position_component;
        let counter_window_static_transform_component;
        let counter_window_closed_timer_option;
        let children;

        match counter_window_components_result {
            Ok(result) => {
                counter_window_component = result.0;
                counter_window_rigid_body_position_component = result.1;
                counter_window_static_transform_component = result.2;
                counter_window_closed_timer_option = result.5;
                children = result.7;
            }
            Err(_err) => {
                continue;
            }
        }

        match counter_window_component.locked_status {
            LockedStatus::Open => {}
            LockedStatus::Closed => {
                continue;
            }
            LockedStatus::None => {}
        }

        let mut opened_sensor = Entity::from_bits(0);

        for child in children.iter() {
            opened_sensor = *child;
            break;
            // Should only have one child.
        }

        let counter_window_sensor_components_result =
            counter_window_sensor_query.get_component::<CounterWindowSensor>(opened_sensor);
        let counter_window_sensor_component;

        match counter_window_sensor_components_result {
            Ok(counter_window_sensor) => {
                counter_window_sensor_component = counter_window_sensor;
            }
            Err(_err) => {
                continue;
            }
        }

        let mut pawn_has_permission = false;

        match request.opener_option {
            Some(opener) => {
                let pawn_space_access_component_result =
                    pawn_query.get_component::<ShipAuthorization>(opener);
                let pawn_space_access_component;

                match pawn_space_access_component_result {
                    Ok(result) => {
                        pawn_space_access_component = result;
                    }
                    Err(_err) => {
                        continue;
                    }
                }

                for space_permission in &counter_window_component.access_permissions {
                    if pawn_space_access_component
                        .access
                        .contains(space_permission)
                        == true
                    {
                        pawn_has_permission = true;
                        break;
                    }
                }
            }
            None => {
                pawn_has_permission = true;
            }
        }

        match counter_window_closed_timer_option {
            Some(mut counter_window_closed_timer) => {
                counter_window_closed_timer.timer.pause();
                counter_window_closed_timer.timer.reset();
            }
            None => {}
        }

        if pawn_has_permission == true {
            if !matches!(counter_window_component.status, CounterWindowStatus::Open) {
                let sfx_entity = commands
                    .spawn()
                    .insert_bundle(CounterWindowOpenSfxBundle::new(
                        counter_window_static_transform_component.transform,
                    ))
                    .id();
                sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
            }

            let cell_id = world_to_cell_id(
                counter_window_rigid_body_position_component
                    .translation
                    .into(),
            );
            let cell_id2 = Vec2Int {
                x: cell_id.x,
                y: cell_id.z,
            };
            if AtmosphericsResource::is_id_out_of_range(cell_id2) {
                continue;
            }
            let atmos_id = get_atmos_index(cell_id2);
            let atmospherics = atmospherics_resource
                .atmospherics
                .get_mut(atmos_id)
                .unwrap();

            atmospherics.blocked = false;
            atmospherics.forces_push_up = true;

            counter_window_component.status = CounterWindowStatus::Open;
            counter_window_component.access_lights = CounterWindowAccessLightsStatus::Granted;

            let mut counter_window_rigid_body_position =
                counter_window_rigid_body_position_component.clone();

            counter_window_rigid_body_position.translation.y = 2.943;

            counter_window_rigid_body_position_component.translation =
                counter_window_rigid_body_position.translation;

            commands
                .entity(counter_window_sensor_component.parent)
                .insert(CounterWindowOpenTimer::default());
        } else {
            counter_window_component.access_lights = CounterWindowAccessLightsStatus::Denied;

            commands
                .entity(counter_window_sensor_component.parent)
                .insert(CounterWindowDeniedTimer::default());

            let sfx_entity = commands
                .spawn()
                .insert_bundle(CounterWindowDeniedSfxBundle::new(
                    counter_window_static_transform_component.transform,
                ))
                .id();
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        }
    }

    for request in close_requests {
        match counter_window_query.get_mut(request.interacted) {
            Ok((
                mut counter_window_component,
                rigid_body_position_component,
                _static_transform_component,
                _counter_window_open_timer_option,
                _counter_window_denied_timer_option,
                _counter_window_closed_timer_option,
                counter_window_entity,
                _children_component,
                _examinable_component,
            )) => {
                match counter_window_component.locked_status {
                    LockedStatus::Open => {
                        continue;
                    }
                    LockedStatus::Closed => {}
                    LockedStatus::None => {}
                }

                let mut pawn_has_permission = false;

                match request.interacter_option {
                    Some(interacter) => {
                        let pawn_space_access_component_result =
                            pawn_query.get_component::<ShipAuthorization>(interacter);
                        let pawn_space_access_component;

                        match pawn_space_access_component_result {
                            Ok(result) => {
                                pawn_space_access_component = result;
                            }
                            Err(_err) => {
                                continue;
                            }
                        }

                        for space_permission in &counter_window_component.access_permissions {
                            if pawn_space_access_component
                                .access
                                .contains(space_permission)
                                == true
                            {
                                pawn_has_permission = true;
                                break;
                            }
                        }
                    }
                    None => {
                        pawn_has_permission = true;
                    }
                }

                if pawn_has_permission == false {
                    continue;
                }

                counter_window_component.status = CounterWindowStatus::Closed;

                let cell_id = world_to_cell_id(rigid_body_position_component.translation.into());
                let cell_id2 = Vec2Int {
                    x: cell_id.x,
                    y: cell_id.z,
                };
                if AtmosphericsResource::is_id_out_of_range(cell_id2) {
                    continue;
                }
                let atmos_id = get_atmos_index(cell_id2);
                let atmospherics = atmospherics_resource
                    .atmospherics
                    .get_mut(atmos_id)
                    .unwrap();

                atmospherics.blocked = true;
                atmospherics.forces_push_up = false;

                commands
                    .entity(counter_window_entity)
                    .insert(CounterWindowClosedTimer::default());
            }
            Err(_rr) => {}
        }
    }
}
