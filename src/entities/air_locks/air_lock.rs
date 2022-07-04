use bevy::{core::Timer, prelude::Component};

use crate::core::pawn::pawn::ShipAuthorizationEnum;

#[derive(Component)]
pub struct AirLock {
    pub status: AirLockStatus,
    pub access_lights: AccessLightsStatus,
    pub access_permissions: Vec<ShipAuthorizationEnum>,
    pub locked_status: LockedStatus,

    pub denied_timer_option: Option<Timer>,
    pub open_timer_option: Option<Timer>,
    pub closed_timer_option: Option<Timer>,
}

pub enum AirLockStatus {
    Open,
    Closed,
}

pub enum AccessLightsStatus {
    Neutral,
    Granted,
    Denied,
}

impl Default for AirLock {
    fn default() -> Self {
        Self {
            status: AirLockStatus::Closed,
            access_lights: AccessLightsStatus::Neutral,
            access_permissions: vec![ShipAuthorizationEnum::Common],
            locked_status: LockedStatus::None,
            denied_timer_option: None,
            open_timer_option: None,
            closed_timer_option: None,
        }
    }
}

pub enum LockedStatus {
    Open,
    Closed,
    None,
}

pub fn open_timer() -> Timer {
    Timer::from_seconds(5.0, false)
}

pub fn denied_timer() -> Timer {
    Timer::from_seconds(5.0, false)
}

pub fn closed_timer() -> Timer {
    Timer::from_seconds(1.1, false)
}
