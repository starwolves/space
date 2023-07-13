use airlocks::airlock_events::{AirlockCloseRequest, LockedStatus};
use bevy::{
    hierarchy::Children,
    prelude::{
        info, warn, Commands, Component, Entity, Event, EventReader, Query, ResMut, Transform, With,
    },
    time::Timer,
};
use bevy_xpbd_3d::prelude::{Collider, CollisionLayers};
use entity::{entity_data::EntityGroup, examine::Examinable};
use networking::server::NetworkingChatServerMessage;
use pawn::pawn::{Pawn, ShipAuthorization, ShipAuthorizationEnum};
use physics::physics::{get_bit_masks, ColliderGroup};
use sfx::{builder::sfx_builder, entity_update::SfxAutoDestroyTimers};
use sounds::{
    counter_window::{
        counter_window_closed_sfx::CounterWindowClosedSfxBundle,
        counter_window_denied_sfx::CounterWindowDeniedSfxBundle,
        counter_window_open_sfx::CounterWindowOpenSfxBundle,
    },
    shared::sfx_auto_destroy,
};
use text_api::core::{FURTHER_ITALIC_FONT, WARNING_COLOR};

use bevy::prelude::EventWriter;
use networking::server::OutgoingReliableServerMessage;
/// Open counter window request event.

pub struct CounterWindowOpenRequest {
    pub opener_option: Option<Entity>,
    pub opened: Entity,
}

/// Process counter windows events.

pub(crate) fn counter_window_events(
    mut counter_window_sensor_collisions: EventReader<CounterWindowSensorCollision>,
    mut counter_window_toggle_open_action: EventReader<InputCounterWindowToggleOpen>,
    mut counter_window_query: Query<(
        &mut CounterWindow,
        &Transform,
        Entity,
        &Children,
        &mut Examinable,
    )>,
    mut counter_window_colliders: Query<&mut CollisionLayers, With<Collider>>,
    counter_window_sensor_query: Query<&CounterWindowSensor>,
    pawn_query: Query<(&Pawn, &ShipAuthorization)>,
    mut auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
    mut counter_window_lock_open_events: EventReader<CounterWindowLockOpen>,
    mut counter_window_lock_close_events: EventReader<CounterWindowLockClosed>,
    mut unlock_events: EventReader<CounterWindowUnlock>,
    mut server: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
) {
    let mut close_requests = vec![];
    let mut open_requests = vec![];

    for event in unlock_events.iter() {
        match counter_window_query.get_mut(event.locked) {
            Ok((
                mut counter_window_component,
                _rigid_body_position_component,
                _counter_window_entity,
                _children_component,
                mut examinable_component,
            )) => {
                counter_window_component.locked_status = LockedStatus::None;
                counter_window_component.access_lights = CounterWindowAccessLightsStatus::Neutral;

                match counter_window_component.status {
                    CounterWindowStatus::Open => {
                        close_requests.push(AirlockCloseRequest {
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
                        server.send(OutgoingReliableServerMessage {
                            handle: t,
                            message: NetworkingChatServerMessage::ChatMessage(personal_update_text),
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
                        server.send(OutgoingReliableServerMessage {
                            handle: t,
                            message: NetworkingChatServerMessage::ChatMessage(personal_update_text),
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
                _counter_window_entity,
                _children_component,
                mut examinable_component,
            )) => {
                counter_window_component.locked_status = LockedStatus::Closed;
                match counter_window_component.status {
                    CounterWindowStatus::Open => {
                        close_requests.push(AirlockCloseRequest {
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
                        server.send(OutgoingReliableServerMessage {
                            handle: t,
                            message: NetworkingChatServerMessage::ChatMessage(personal_update_text),
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
        rigid_body_position_component,
        counter_window_entity,
        children_component,
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

        match counter_window_component.open_timer.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();

                    close_requests.push(AirlockCloseRequest {
                        interacter_option: None,
                        interacted: counter_window_entity,
                    });
                }
            }
            None => {}
        }

        let mut collider_option = None;

        for child in children_component.iter() {
            match counter_window_colliders.get(*child) {
                Ok(_) => {
                    collider_option = Some(child);
                }
                Err(_) => {}
            }
        }

        let collider;

        match collider_option {
            Some(e) => {
                collider = e;
            }
            None => {
                warn!("Couldnt find collider of counterWindow.");
                continue;
            }
        }

        let mut collision_groups;

        match counter_window_colliders.get_mut(*collider) {
            Ok(c) => {
                collision_groups = c;
            }
            Err(_rr) => {
                warn!("Couldnt find collider child of counterWindow.");
                continue;
            }
        }

        match counter_window_component.closed_timer.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();

                    let masks = get_bit_masks(ColliderGroup::Standard);

                    *collision_groups = CollisionLayers::from_bits(masks.0, masks.1);

                    counter_window_component.access_lights =
                        CounterWindowAccessLightsStatus::Neutral;

                    let sfx_entity = sfx_builder(
                        &mut commands,
                        *rigid_body_position_component,
                        Box::new(CounterWindowClosedSfxBundle::new),
                    );
                    sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
                }
            }
            None => {}
        }

        match counter_window_component.denied_timer.as_mut() {
            Some(timer_component) => {
                if timer_component.finished() == true {
                    timer_component.pause();
                    timer_component.reset();

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
        let opened_entity = event.opened;

        info!("Toggle open action from {:?}", opened_entity);

        match counter_window_query.get(opened_entity) {
            Ok((
                counter_window_component,
                _rigid_body_position_component,
                _counter_window_entity,
                _children_component,
                _examinable_component,
            )) => {
                match counter_window_component.status {
                    CounterWindowStatus::Open => {
                        close_requests.push(AirlockCloseRequest {
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
        let counter_window_rigid_body_position_component;
        let children;

        match counter_window_components_result {
            Ok(result) => {
                counter_window_component = result.0;
                counter_window_rigid_body_position_component = result.1;
                children = result.3;
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

        let mut opened_sensor_option = None;

        for child in children.iter() {
            match counter_window_sensor_query.get(*child) {
                Ok(_) => {
                    opened_sensor_option = Some(*child);

                    break;
                }
                Err(_) => {}
            }
        }

        let _opened_sensor;

        match opened_sensor_option {
            Some(t) => {
                _opened_sensor = t;
            }
            None => {
                warn!("Couldnt find child yo!");
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

        match counter_window_component.closed_timer.as_mut() {
            Some(counter_window_closed_timer) => {
                counter_window_closed_timer.pause();
                counter_window_closed_timer.reset();
            }
            None => {}
        }

        let mut collider_option = None;

        for child in children.iter() {
            match counter_window_colliders.get(*child) {
                Ok(_) => {
                    collider_option = Some(child);
                }
                Err(_rr) => {}
            }
        }

        let collider;

        match collider_option {
            Some(c) => {
                collider = c;
            }
            None => {
                warn!("Couldnt find counterwindow child!");
                continue;
            }
        }

        let mut collision_groups;

        match counter_window_colliders.get_mut(*collider) {
            Ok(c) => {
                collision_groups = c;
            }
            Err(_rr) => {
                warn!("Couldnt find counterwindow collision component.");
                continue;
            }
        }

        if pawn_has_permission == true {
            if !matches!(counter_window_component.status, CounterWindowStatus::Open) {
                let sfx_entity = sfx_builder(
                    &mut commands,
                    *counter_window_rigid_body_position_component,
                    Box::new(CounterWindowOpenSfxBundle::new),
                );
                sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
            }

            counter_window_component.status = CounterWindowStatus::Open;
            counter_window_component.access_lights = CounterWindowAccessLightsStatus::Granted;

            let masks = get_bit_masks(ColliderGroup::NoCollision);

            *collision_groups = CollisionLayers::from_bits(masks.0, masks.1);

            counter_window_component.open_timer = Some(open_timer())
        } else {
            counter_window_component.access_lights = CounterWindowAccessLightsStatus::Denied;

            counter_window_component.denied_timer = Some(denied_timer());

            let sfx_entity = sfx_builder(
                &mut commands,
                *counter_window_rigid_body_position_component,
                Box::new(CounterWindowDeniedSfxBundle::new),
            );
            sfx_auto_destroy(sfx_entity, &mut auto_destroy_timers);
        }
    }

    for request in close_requests {
        match counter_window_query.get_mut(request.interacted) {
            Ok((
                mut counter_window_component,
                _rigid_body_position_component,
                _counter_window_entity,
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

                counter_window_component.closed_timer = Some(close_timer());
            }
            Err(_rr) => {}
        }
    }
}

/// The component for the physics sensor.
#[derive(Component)]

pub struct CounterWindowSensor {
    pub parent: Entity,
}

impl Default for CounterWindowSensor {
    fn default() -> Self {
        Self {
            parent: Entity::from_raw(0),
        }
    }
}

/// The component.
#[derive(Component)]

pub struct CounterWindow {
    /// State.
    pub status: CounterWindowStatus,
    /// State of access lights.
    pub access_lights: CounterWindowAccessLightsStatus,
    /// Authorization required to interact.
    pub access_permissions: Vec<ShipAuthorizationEnum>,
    /// Lock state of counter window.
    pub locked_status: LockedStatus,

    pub(crate) denied_timer: Option<Timer>,
    pub(crate) open_timer: Option<Timer>,
    pub(crate) closed_timer: Option<Timer>,
}

pub enum CounterWindowStatus {
    Open,
    Closed,
}

pub enum CounterWindowAccessLightsStatus {
    Neutral,
    Granted,
    Denied,
}

impl Default for CounterWindow {
    fn default() -> Self {
        Self {
            status: CounterWindowStatus::Closed,
            access_lights: CounterWindowAccessLightsStatus::Neutral,
            access_permissions: vec![ShipAuthorizationEnum::Common],
            locked_status: LockedStatus::None,
            denied_timer: None,
            open_timer: None,
            closed_timer: None,
        }
    }
}
use bevy::time::TimerMode;

/// Create a timer.

pub fn open_timer() -> Timer {
    Timer::from_seconds(5.0, TimerMode::Once)
}
/// Create a timer.

pub fn close_timer() -> Timer {
    Timer::from_seconds(1.1, TimerMode::Once)
}
/// Create a timer.

pub fn denied_timer() -> Timer {
    Timer::from_seconds(5.0, TimerMode::Once)
}

/// Counter window sensor collision event.
#[derive(Event)]
pub struct CounterWindowSensorCollision {
    pub collider1_entity: Entity,
    pub collider2_entity: Entity,

    pub collider1_group: EntityGroup,
    pub collider2_group: EntityGroup,

    pub started: bool,
}

/// Counter window toggle open event.
#[derive(Event)]
pub struct InputCounterWindowToggleOpen {
    pub handle_option: Option<u64>,

    pub opener: Entity,
    pub opened: Entity,
}
/// Counter window lock open event.
#[derive(Event)]
pub struct CounterWindowLockOpen {
    pub handle_option: Option<u64>,

    pub locked: Entity,
    pub locker: Entity,
}

/// Counter window lock closed event.
#[derive(Event)]
pub struct CounterWindowLockClosed {
    pub handle_option: Option<u64>,

    pub locked: Entity,
    pub locker: Entity,
}

/// Counter window unlock event.
#[derive(Event)]
pub struct CounterWindowUnlock {
    pub handle_option: Option<u64>,

    pub locked: Entity,
    pub locker: Entity,
}
