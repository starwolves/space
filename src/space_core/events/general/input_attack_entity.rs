use bevy::prelude::Entity;

pub struct InputAttackEntity {
    pub handle : u32,
    pub entity : Entity,
    pub target_entity_bits : u64,
}
