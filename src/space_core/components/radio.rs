use bevy::prelude::Component;

#[derive(Component)]
pub struct Radio {
    pub listen_access : Vec<RadioChannel>,
    pub speak_access : Vec<RadioChannel>
}

#[derive(PartialEq, Debug, Clone)]
pub enum RadioChannel {
    Proximity,
    ProximityEmote,
    Global,
    Common,
    Security,
    SpecialOps
}
