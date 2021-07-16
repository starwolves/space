use bevy::prelude::{EventReader, EventWriter, Query};

use crate::space_core::{components::{entity_data::EntityData, inventory::{Inventory}, inventory_item::InventoryItem,  world_mode::{WorldMode, WorldModes}}, events::{general::wear_item::WearItem, net::net_wear_item::NetWearItem}, structs::network_messages::ReliableServerMessage};

pub fn wear_item(
    mut wear_item_events : EventReader<WearItem>,
    mut inventory_entities : Query<
        &mut Inventory,
    >,
    mut wearable_entities : Query<(
        &InventoryItem,
        &mut WorldMode,
        &EntityData,
    )>,
    mut net_wear_item : EventWriter<NetWearItem>,
) {

    // Todo for robustness against script kids:
    // Check if wear_slot string provided by client is legit to the item it submitted to that slot. Ie Jumpsuit cant have "helmet".

    for event in wear_item_events.iter() {


        let wearer_components_option = inventory_entities.get_mut(event.wearer_entity);
        let wearer_components;

        match wearer_components_option {
            Ok(components) => {
                wearer_components = components;
            },
            Err(_rr) => {
                continue;
            },
        }


        let mut wearer_inventory = wearer_components;

        let pickup_slot_name = wearer_inventory.pickup_slot.clone();

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
            },
            None => {
                continue;
            },
        }


        let wearable_entity;
        
        match pickup_slot.slot_item {
            Some(item) => {
                wearable_entity = item;
            },
            None => {
                continue;
            },
        }



        let wearable_components_option = wearable_entities.get_mut(wearable_entity);
        let mut wearable_components;

        match wearable_components_option {
            Ok(wearable) => {
                wearable_components = wearable;
            },
            Err(_rr) => {
                continue;
            },
        }

        let wearable_wearable = wearable_components.0;


        match wear_slot.slot_item {
            Some(_) => {
                continue;
            },
            None => {
                
            },
        }
        


        let _wear_slot_type = wear_slot.slot_type;


        if !matches!(&wearable_wearable.slot_type, _wear_slot_type) {
            continue;
        }

        
        pickup_slot.slot_item = None;
        wear_slot.slot_item = Some(wearable_entity);
        wearable_components.1.mode = WorldModes::Worn;

        net_wear_item.send(NetWearItem {
            handle: event.handle,
            message: ReliableServerMessage::PickedUpItem(wearable_components.2.entity_type.clone(), wearable_entity.to_bits(), wear_slot.slot_name.clone()),
        });


    }    


}
