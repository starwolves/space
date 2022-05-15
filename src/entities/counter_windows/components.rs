use bevy_core::Timer;
use bevy_ecs::{entity::Entity, prelude::Component};

use crate::{
    core::pawn::components::ShipAuthorizationEnum, entities::air_locks::components::LockedStatus,
};

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

#[derive(Component)]
pub struct CounterWindow {
    pub status: CounterWindowStatus,
    pub access_lights: CounterWindowAccessLightsStatus,
    pub access_permissions: Vec<ShipAuthorizationEnum>,
    pub locked_status: LockedStatus,

    pub denied_timer: Option<Timer>,
    pub open_timer: Option<Timer>,
    pub closed_timer: Option<Timer>,
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

pub fn open_timer() -> Timer {
    Timer::from_seconds(5.0, false)
}

pub fn close_timer() -> Timer {
    Timer::from_seconds(1.1, false)
}

pub fn denied_timer() -> Timer {
    Timer::from_seconds(5.0, false)
}
