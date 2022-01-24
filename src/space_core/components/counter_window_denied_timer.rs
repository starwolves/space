use bevy::{core::Timer, prelude::Component};
#[derive(Component)]
pub struct CounterWindowDeniedTimer {
    pub timer : Timer
}

impl Default for CounterWindowDeniedTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, false),
        }
    }
}
