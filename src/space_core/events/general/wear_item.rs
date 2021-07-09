use bevy::prelude::Entity;

pub struct WearItem {
    pub handle : u32,
    pub wearer_entity : Entity,
    pub wearable_id : u32,
    pub wear_slot : String,
}
