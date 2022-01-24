use bevy::{core::Timer, prelude::Component};
#[derive(Component)]
pub struct AirLockDeniedTimer {
    pub timer : Timer
}

impl Default for AirLockDeniedTimer {
    fn default() -> Self {
        Self {
            timer : Timer::from_seconds(5.0, false),
        }
    }
}
