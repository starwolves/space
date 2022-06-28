use bevy_ecs::entity::Entity;
use bevy_math::Vec3;

use crate::core::networking::resources::ReliableServerMessage;

pub struct InputDropCurrentItem {
    pub pickuper_entity: Entity,
    pub input_position_option: Option<Vec3>,
}

pub struct InputThrowItem {
    pub entity: Entity,
    pub position: Vec3,
    pub angle: f32,
}

pub struct InputSwitchHands {
    pub entity: Entity,
}

pub struct InputTakeOffItem {
    pub entity: Entity,
    pub slot_name: String,
}

pub struct InputUseWorldItem {
    pub pickuper_entity: Entity,
    pub pickupable_entity_bits: u64,
}

pub struct InputWearItem {
    pub wearer_entity: Entity,
    pub wearable_id_bits: u64,
    pub wear_slot: String,
}

pub struct NetDropCurrentItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetPickupWorldItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetSwitchHands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetTakeOffItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetThrowItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetWearItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
