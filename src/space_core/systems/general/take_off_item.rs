use bevy::prelude::{EventReader, EventWriter, Query};

use crate::space_core::{components::{entity_data::EntityData, inventory::Inventory, inventory_item::InventoryItem, world_mode::{WorldMode, WorldModes}}, events::{general::take_off_item::TakeOffItem, net::net_takeoff_item::NetTakeOffItem}, structs::network_messages::ReliableServerMessage};

pub fn take_off_item(
    mut take_off_item_events : EventReader<TakeOffItem>,
    mut inventory_entities : Query<
        &mut Inventory,
    >,
    mut pickupable_entities : Query<(
        &InventoryItem,
        &mut WorldMode,
        &EntityData
    )>,
    mut net_takeoff_item : EventWriter<NetTakeOffItem>,
) {


    for event in take_off_item_events.iter() {

        let carrier_components_option = inventory_entities.get_mut(event.entity);
        let carrier_components;

        match carrier_components_option {
            Ok(components) => {
                carrier_components = components;
            },
            Err(_rr) => {
                continue;
            },
        }



        let mut carrier_inventory = carrier_components;
        let pickup_slot_name = carrier_inventory.pickup_slot.clone();

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
            },
            None => {
                continue;
            },
        }

        match pickup_slot.slot_item {
            Some(_) => {continue;},
            None => {},
        }


        let takeoff_components_option = pickupable_entities.get_mut(takeoff_entity);
        let takeoff_components;

        match takeoff_components_option {
            Ok(components) => {
                takeoff_components = components;
            },
            Err(_rr) => {
                continue;
            },
        }

        let mut takeoff_worldmode = takeoff_components.1;

        pickup_slot.slot_item = Some(takeoff_entity);
        takeoff_slot.slot_item = None;
        takeoff_worldmode.mode = WorldModes::Held;



        net_takeoff_item.send(NetTakeOffItem {
            handle: event.handle,
            message: ReliableServerMessage::EquippedWornItem(takeoff_components.2.entity_type.clone(), takeoff_entity.to_bits(), takeoff_slot.slot_name.clone()),
        });

        




    }


}
