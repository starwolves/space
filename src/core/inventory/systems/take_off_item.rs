use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::{Query, Res},
};

use crate::core::{
    connected_player::resources::HandleToEntity,
    entity::components::EntityData,
    inventory::{
        components::Inventory,
        events::{InputTakeOffItem, NetTakeOffItem},
    },
    inventory_item::components::InventoryItem,
    networking::resources::ReliableServerMessage,
    physics::components::{WorldMode, WorldModes},
};

pub fn take_off_item(
    mut take_off_item_events: EventReader<InputTakeOffItem>,
    mut inventory_entities: Query<&mut Inventory>,
    mut pickupable_entities: Query<(&InventoryItem, &mut WorldMode, &EntityData)>,
    mut net_takeoff_item: EventWriter<NetTakeOffItem>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in take_off_item_events.iter() {
        let carrier_components_option = inventory_entities.get_mut(event.entity);
        let carrier_components;

        match carrier_components_option {
            Ok(components) => {
                carrier_components = components;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut carrier_inventory = carrier_components;
        let pickup_slot_name = carrier_inventory.active_slot.clone();

        let mut pickup_slot_option = None;
        let mut take_off_slot_option = None;

        for slot in carrier_inventory.slots.iter_mut() {
            if slot.slot_name == pickup_slot_name {
                pickup_slot_option = Some(slot);
            } else if slot.slot_name == event.slot_name {
                take_off_slot_option = Some(slot);
            }
        }

        let pickup_slot = pickup_slot_option.unwrap();
        let takeoff_slot = take_off_slot_option.unwrap();

        let takeoff_entity;

        match takeoff_slot.slot_item {
            Some(item) => {
                takeoff_entity = item;
            }
            None => {
                continue;
            }
        }

        match pickup_slot.slot_item {
            Some(_) => {
                continue;
            }
            None => {}
        }

        let takeoff_components_option = pickupable_entities.get_mut(takeoff_entity);
        let takeoff_components;

        match takeoff_components_option {
            Ok(components) => {
                takeoff_components = components;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut takeoff_worldmode = takeoff_components.1;

        pickup_slot.slot_item = Some(takeoff_entity);
        takeoff_slot.slot_item = None;
        takeoff_worldmode.mode = WorldModes::Held;

        match handle_to_entity.inv_map.get(&event.entity) {
            Some(handle) => {
                net_takeoff_item.send(NetTakeOffItem {
                    handle: *handle,
                    message: ReliableServerMessage::EquippedWornItem(
                        takeoff_components.2.entity_name.clone(),
                        takeoff_entity.to_bits(),
                        takeoff_slot.slot_name.clone(),
                    ),
                });
            }
            None => {}
        }
    }
}
