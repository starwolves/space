use bevy_core::Timer;
use bevy_internal::prelude::Component;

use crate::space::core::pawn::components::SpaceAccessEnum;

#[derive(Component)]
pub struct AirLock {
    pub status: AirLockStatus,
    pub access_lights: AccessLightsStatus,
    pub access_permissions: Vec<SpaceAccessEnum>,
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
            access_permissions: vec![SpaceAccessEnum::Common],
        }
    }
}

#[derive(Component)]
pub struct AirLockOpenTimer {
    pub timer: Timer,
}

impl Default for AirLockOpenTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, false),
        }
    }
}

#[derive(Component)]
pub struct AirLockDeniedTimer {
    pub timer: Timer,
}

impl Default for AirLockDeniedTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, false),
        }
    }
}

#[derive(Component)]
pub struct AirLockClosedTimer {
    pub timer: Timer,
}

impl Default for AirLockClosedTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.1, false),
        }
    }
}
