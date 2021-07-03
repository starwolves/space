pub struct WorldMode {
    pub mode : WorldModes
}

pub enum WorldModes {
    Static,
    Kinematic,
    Physics,
    Held,
    Worn,
}
