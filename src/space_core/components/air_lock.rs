use crate::space_core::enums::space_access::SpaceAccess;

pub struct AirLock {
    pub status : AirLockStatus,
    pub access_lights : AccessLightsStatus,
    pub access_permissions : Vec<SpaceAccess>
}

pub enum AirLockStatus {
    Open,
    Closed
}

pub enum AccessLightsStatus {
    Neutral
}
