use bevy::prelude::Entity;

pub struct InputWearItem {
    pub handle : u32,
    pub wearer_entity : Entity,
    pub wearable_id_bits : u64,
    pub wear_slot : String,
}
