use bevy::{core::Timer, prelude::{Component, Entity}};

use crate::space_core::ecs::pawn::components::SpaceAccessEnum;

#[derive(Component)]
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

#[derive(Component)]
pub struct CounterWindowSensor {

    pub parent : Entity

}

impl Default for CounterWindowSensor {
    fn default() -> Self {
        Self {
            parent : Entity::from_raw(0),
        }
    }
}



#[derive(Component)]
pub struct CounterWindow {
    pub status : CounterWindowStatus,
    pub access_lights : CounterWindowAccessLightsStatus,
    pub access_permissions : Vec<SpaceAccessEnum>
}

pub enum CounterWindowStatus {
    Open,
    Closed
}

pub enum CounterWindowAccessLightsStatus {
    Neutral,
    Granted,
    Denied
}

impl Default for CounterWindow {
    fn default() -> Self {
        Self {
            status: CounterWindowStatus::Closed,
            access_lights: CounterWindowAccessLightsStatus::Neutral,
            access_permissions: vec![SpaceAccessEnum::Common],
        }
    }
}
