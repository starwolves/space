use bevy::{math::Vec3, prelude::Entity};

use crate::space::core::networking::resources::ReliableServerMessage;

pub struct InputDropCurrentItem {
    pub handle : u32,
    pub pickuper_entity : Entity,
    pub input_position_option : Option<Vec3>,
}

pub struct InputThrowItem {
    pub handle : u32,
    pub entity : Entity,
    pub position : Vec3,
    pub angle : f32,
}

pub struct InputSwitchHands {
    pub handle : u32,
    pub entity : Entity,
}

pub struct InputTakeOffItem {
    pub handle : u32,
    pub entity : Entity,
    pub slot_name : String,
}

pub struct InputUseWorldItem{
    pub handle : u32,
    pub pickuper_entity : Entity,
    pub pickupable_entity_bits : u64,
}

pub struct InputWearItem {
    pub handle : u32,
    pub wearer_entity : Entity,
    pub wearable_id_bits : u64,
    pub wear_slot : String,
}

pub struct NetDropCurrentItem {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetPickupWorldItem {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetSwitchHands {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetTakeOffItem {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetThrowItem {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetWearItem {
    pub handle : u32,
    pub message : ReliableServerMessage
}
