pub struct Radio {
    pub listen_access : Vec<RadioChannel>,
    pub speak_access : Vec<RadioChannel>
}

#[derive(PartialEq, Debug)]
pub enum RadioChannel {
    Proximity,
    ProximityEmote,
    OOC,
    Common,
    Security,
    SpecialOps
}
