pub struct Radio {
    pub listen_access : Vec<RadioChannel>,
    pub speak_access : Vec<RadioChannel>
}

#[derive(PartialEq)]
pub enum RadioChannel {
    Proximity,
    ProximityEmote,
    Common,
    Security,
    SpecialOps
}
