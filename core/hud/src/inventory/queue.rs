use bevy::prelude::{EventReader, EventWriter, ResMut, Resource};
use inventory::{
    client::slots::AddedSlot, net::InventoryServerMessage, server::inventory::ItemAddedToSlot,
};
use networking::client::IncomingReliableServerMessage;

use super::{items::HudAddItemToSlot, slots::HudAddInventorySlot};

pub struct RequeueHudAddItemToSlot {
    pub queued: HudAddItemToSlot,
}
pub(crate) fn queue_inventory_updates(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut queue: ResMut<InventoryUpdatesQueue>,
    mut added_slot_events: EventReader<AddedSlot>,
) {
    for event in added_slot_events.iter() {
        queue.slot_updates.push(event.clone());
    }
    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(item) => {
                queue.item_updates.push(item.clone());
            }
            _ => (),
        }
    }
}
/// Resource that queues inventory updates. For when we receive them before the client has fully initialized the inventory and UI.
#[derive(Resource, Clone, Default)]
pub struct InventoryUpdatesQueue {
    pub flushed: bool,
    pub item_updates: Vec<ItemAddedToSlot>,
    pub slot_updates: Vec<AddedSlot>,
}

pub(crate) fn inventory_net_updates(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut queue: ResMut<InventoryUpdatesQueue>,
    mut slot_event: EventWriter<HudAddInventorySlot>,
    mut item_event: EventWriter<HudAddItemToSlot>,
    mut added_slot_events: EventReader<AddedSlot>,
) {
    let mut to_be_added_slot_ids = vec![];
    let mut to_be_added_items = vec![];

    if queue.flushed == false {
        queue.flushed = true;

        for slot in queue.slot_updates.clone() {
            slot_event.send(HudAddInventorySlot { slot: slot.clone() });
            to_be_added_slot_ids.push(slot.id);
        }

        for item in queue.item_updates.clone() {
            item_event.send(HudAddItemToSlot { item: item.clone() });
            to_be_added_items.push((item.item_entity, item.slot_id));
        }

        queue.item_updates.clear();
        queue.slot_updates.clear();
    }

    for event in added_slot_events.iter() {
        if to_be_added_slot_ids.contains(&event.id) {
            continue;
        }
        slot_event.send(HudAddInventorySlot {
            slot: event.clone(),
        });
    }

    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(item) => {
                let mut found = false;
                for (entity, slot_id) in to_be_added_items.iter() {
                    if entity == &item.item_entity && slot_id == &item.slot_id {
                        found = true;
                        break;
                    }
                }
                if found {
                    continue;
                }
                item_event.send(HudAddItemToSlot { item: item.clone() });
            }
            _ => (),
        }
    }
}
