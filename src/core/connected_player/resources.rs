use std::collections::HashMap;

use bevy_ecs::entity::Entity;

#[derive(Default)]
pub struct HandleToEntity {
    pub map: HashMap<u64, Entity>,
    pub inv_map: HashMap<Entity, u64>,
}
