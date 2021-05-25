use crate::space_core::enums::space_access_enum::SpaceAccessEnum;

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
