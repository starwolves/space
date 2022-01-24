use bevy::prelude::Component;

use super::pawn::SpaceAccessEnum;


#[derive(Component)]
pub struct AirLock {
    pub status : AirLockStatus,
    pub access_lights : AccessLightsStatus,
    pub access_permissions : Vec<SpaceAccessEnum>
}

pub enum AirLockStatus {
    Open,
    Closed
}

pub enum AccessLightsStatus {
    Neutral,
    Granted,
    Denied
}

impl Default for AirLock {
    fn default() -> Self {
        Self {
            status : AirLockStatus::Closed,
            access_lights : AccessLightsStatus::Neutral,
            access_permissions : vec![SpaceAccessEnum::Common]
        }
    }
}
