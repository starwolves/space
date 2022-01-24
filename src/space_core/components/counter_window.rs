use bevy::prelude::Component;

use super::pawn::SpaceAccessEnum;

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
