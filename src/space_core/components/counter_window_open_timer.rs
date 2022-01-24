use bevy::{core::Timer, prelude::Component};
#[derive(Component)]
pub struct CounterWindowOpenTimer {
    pub timer : Timer
}

impl Default for CounterWindowOpenTimer {
    fn default() -> Self {
        Self {
            timer : Timer::from_seconds(5.0, false),
        }
    }
}
