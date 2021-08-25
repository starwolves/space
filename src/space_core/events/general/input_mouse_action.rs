use bevy::prelude::Entity;

pub struct InputMouseAction {
    pub handle : u32,
    pub entity : Entity,
    pub pressed : bool,
}
