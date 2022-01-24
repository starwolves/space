use bevy::{core::Timer, prelude::Component};
#[derive(Component)]
pub struct AirLockClosedTimer {
    pub timer : Timer
}

impl Default for AirLockClosedTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.1, false),
        }
    }
}
