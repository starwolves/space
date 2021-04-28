pub struct WorldMode {
    pub mode : WorldModes
}

#[allow(dead_code)]
pub enum WorldModes {
    Static,
    Kinematic,
    Physics,
    Worn
}
