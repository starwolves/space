use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct WorldMode {
    pub mode: WorldModes,
}

pub enum WorldModes {
    Static,
    Kinematic,
    Physics,
    Held,
    Worn,
}
