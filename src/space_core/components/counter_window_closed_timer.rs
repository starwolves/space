use bevy::core::Timer;
pub struct CounterWindowClosedTimer {
    pub timer : Timer
}

impl Default for CounterWindowClosedTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.1, false),
        }
    }
}
