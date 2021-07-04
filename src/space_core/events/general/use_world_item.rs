use bevy::prelude::Entity;

pub struct UseWorldItem{
    pub handle : u32,
    pub pickuper_entity : Entity,
    pub pickupable_entity_id : u32,
    pub pickupable_entity_generation : u32,
}
