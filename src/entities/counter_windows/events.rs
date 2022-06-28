use bevy_ecs::entity::Entity;

use crate::core::{entity::components::EntityGroup, networking::resources::ReliableServerMessage};

pub struct CounterWindowSensorCollision {
    pub collider1_entity: Entity,
    pub collider2_entity: Entity,

    pub collider1_group: EntityGroup,
    pub collider2_group: EntityGroup,

    pub started: bool,
}

pub struct InputCounterWindowToggleOpen {
    pub handle_option: Option<u64>,

    pub opener: Entity,
    pub opened: u64,
}
pub struct CounterWindowLockOpen {
    pub handle_option: Option<u64>,

    pub locked: Entity,
    pub locker: Entity,
}

pub struct CounterWindowLockClosed {
    pub handle_option: Option<u64>,

    pub locked: Entity,
    pub locker: Entity,
}

pub struct CounterWindowUnlock {
    pub handle_option: Option<u64>,

    pub locked: Entity,
    pub locker: Entity,
}

pub struct NetCounterWindow {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
