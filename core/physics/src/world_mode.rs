use bevy::prelude::Component;

/// World mode component.
#[derive(Component)]
pub struct WorldMode {
    pub mode: WorldModes,
}

/// All world modes.
#[derive(Debug)]
pub enum WorldModes {
    Static,
    Kinematic,
    Physics,
    Held,
    Worn,
}
