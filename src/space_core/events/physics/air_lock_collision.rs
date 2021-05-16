use bevy::prelude::Entity;

use crate::space_core::components::entity_data::EntityGroup;

pub struct AirLockCollision {
    pub collider1_entity : Entity,
    pub collider2_entity : Entity,

    pub collider1_group : EntityGroup,
    pub collider2_group : EntityGroup,

    pub started : bool
}
