use crate::space_core::enums::space_access_enum::SpaceAccessEnum;

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
    Neutral
}
