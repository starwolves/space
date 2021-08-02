use bevy::core::Timer;
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
