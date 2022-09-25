use api::data::LockedStatus;
use bevy::{prelude::Component, time::Timer};
use pawn::pawn::ShipAuthorizationEnum;

/// Air lock component.
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

/// Air lock open or closed status.
pub enum AirLockStatus {
    Open,
    Closed,
}

/// Access lights state.
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

/// Create a timer.
pub fn open_timer() -> Timer {
    Timer::from_seconds(5.0, false)
}
/// Create a timer.
pub fn denied_timer() -> Timer {
    Timer::from_seconds(5.0, false)
}
/// Create a timer.
pub fn closed_timer() -> Timer {
    Timer::from_seconds(1.1, false)
}
