use bevy::core::Timer;
pub struct AirLockOpenTimer {
    pub timer : Timer
}

impl Default for AirLockOpenTimer {
    fn default() -> Self {
        Self {
            timer : Timer::from_seconds(5.0, false),
        }
    }
}
