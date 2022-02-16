use bevy::prelude::Entity;

use crate::space::core::entity::components::EntityGroup;

pub struct CounterWindowSensorCollision {
    pub collider1_entity: Entity,
    pub collider2_entity: Entity,

    pub collider1_group: EntityGroup,
    pub collider2_group: EntityGroup,

    pub started: bool,
}
