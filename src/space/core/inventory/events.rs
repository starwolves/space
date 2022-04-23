use bevy_ecs::entity::Entity;
use bevy_math::Vec3;

use crate::space::core::networking::resources::ReliableServerMessage;

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
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetPickupWorldItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetSwitchHands {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetTakeOffItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetThrowItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetWearItem {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

use bevy_app::EventWriter;
use bevy_ecs::system::Res;

use crate::space::core::tab_actions::resources::QueuedTabActions;

pub fn inventory_actions(
    queue: Res<QueuedTabActions>,

    mut pickup_world_item_event: EventWriter<InputUseWorldItem>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "core/inventory/pickup" {
            if queued.target_entity_option.is_some() {
                pickup_world_item_event.send(InputUseWorldItem {
                    pickuper_entity: queued.player_entity,
                    pickupable_entity_bits: queued.target_entity_option.unwrap(),
                });
            }
        }
    }
}
