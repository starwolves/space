use bevy::prelude::Component;

#[derive(Component)]
pub struct WorldMode {
    pub mode: WorldModes,
}

#[derive(Debug)]
pub enum WorldModes {
    Static,
    Kinematic,
    Physics,
    Held,
    Worn,
}
