use bevy::{prelude::Component, time::Timer};
use pawn::pawn::ShipAuthorizationEnum;

use crate::airlock_events::LockedStatus;

/// Air lock component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Airlock {
    /// Air lock state.
    pub status: AirlockStatus,
    /// Current color of the access lights.
    pub access_lights: AccessLightsStatus,
    /// Required authorization to interact with the air lock.
    pub access_permissions: Vec<ShipAuthorizationEnum>,
    /// Whether the air lock is locked.
    pub locked_status: LockedStatus,

    pub(crate) denied_timer_option: Option<Timer>,
    pub(crate) open_timer_option: Option<Timer>,
    pub(crate) closed_timer_option: Option<Timer>,
}

/// Air lock open or closed status.
#[cfg(feature = "server")]
pub enum AirlockStatus {
    Open,
    Closed,
}

/// Access lights state.
#[cfg(feature = "server")]
pub enum AccessLightsStatus {
    Neutral,
    Granted,
    Denied,
}

#[cfg(feature = "server")]
impl Default for Airlock {
    fn default() -> Self {
        Self {
            status: AirlockStatus::Closed,
            access_lights: AccessLightsStatus::Neutral,
            access_permissions: vec![ShipAuthorizationEnum::Common],
            locked_status: LockedStatus::None,
            denied_timer_option: None,
            open_timer_option: None,
            closed_timer_option: None,
        }
    }
}
use bevy::time::TimerMode;

/// Create a timer.
#[cfg(feature = "server")]
pub fn open_timer() -> Timer {
    Timer::from_seconds(5.0, TimerMode::Once)
}
/// Create a timer.
#[cfg(feature = "server")]
pub fn denied_timer() -> Timer {
    Timer::from_seconds(5.0, TimerMode::Once)
}
/// Create a timer.
#[cfg(feature = "server")]
pub fn closed_timer() -> Timer {
    Timer::from_seconds(1.1, TimerMode::Once)
}
