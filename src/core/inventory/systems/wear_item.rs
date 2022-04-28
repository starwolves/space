use bevy_app::{EventReader, EventWriter};
use bevy_ecs::system::{Query, Res};

use crate::core::{
    connected_player::resources::HandleToEntity,
    entity::components::EntityData,
    inventory::{
        components::Inventory,
        events::{InputWearItem, NetWearItem},
    },
    inventory_item::components::InventoryItem,
    networking::resources::ReliableServerMessage,
    physics::components::{WorldMode, WorldModes},
};

pub fn wear_item(
    mut wear_item_events: EventReader<InputWearItem>,
    mut inventory_entities: Query<&mut Inventory>,
    mut wearable_entities: Query<(&InventoryItem, &mut WorldMode, &EntityData)>,
    mut net_wear_item: EventWriter<NetWearItem>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in wear_item_events.iter() {
        let wearer_components_option = inventory_entities.get_mut(event.wearer_entity);
        let wearer_components;

        match wearer_components_option {
            Ok(components) => {
                wearer_components = components;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut wearer_inventory = wearer_components;

        let pickup_slot_name = wearer_inventory.active_slot.clone();

        let mut pickup_slot_option = None;
        let mut wear_slot_option = None;

        for slot in wearer_inventory.slots.iter_mut() {
            if slot.slot_name == pickup_slot_name {
                pickup_slot_option = Some(slot);
            } else if slot.slot_name == event.wear_slot {
                wear_slot_option = Some(slot);
            }
        }

        let pickup_slot = pickup_slot_option.unwrap();

        let wear_slot;

        match wear_slot_option {
            Some(slot) => {
                wear_slot = slot;
            }
            None => {
                continue;
            }
        }

        let wearable_entity;

        match pickup_slot.slot_item {
            Some(item) => {
                wearable_entity = item;
            }
            None => {
                continue;
            }
        }

        let wearable_components_option = wearable_entities.get_mut(wearable_entity);
        let mut wearable_components;

        match wearable_components_option {
            Ok(wearable) => {
                wearable_components = wearable;
            }
            Err(_rr) => {
                continue;
            }
        }

        let wearable_wearable = wearable_components.0;

        match wear_slot.slot_item {
            Some(_) => {
                continue;
            }
            None => {}
        }

        if wearable_wearable.slot_type != wear_slot.slot_type {
            continue;
        }

        pickup_slot.slot_item = None;
        wear_slot.slot_item = Some(wearable_entity);
        wearable_components.1.mode = WorldModes::Worn;

        match handle_to_entity.inv_map.get(&event.wearer_entity) {
            Some(handle) => {
                net_wear_item.send(NetWearItem {
                    handle: *handle,
                    message: ReliableServerMessage::PickedUpItem(
                        wearable_components.2.entity_name.clone(),
                        wearable_entity.to_bits(),
                        wear_slot.slot_name.clone(),
                    ),
                });
            }
            None => {}
        }
    }
}
