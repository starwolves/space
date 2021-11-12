use bevy::prelude::Entity;

pub struct InputExamineEntity{
    pub handle : u32,
    pub examine_entity_bits : u64,
    pub entity : Entity,
}
