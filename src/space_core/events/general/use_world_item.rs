use bevy::prelude::Entity;

pub struct UseWorldItem{
    pub handle : u32,
    pub pickuper_entity : Entity,
    pub pickupable_entity_bits : u64,
}
