use bevy::prelude::{warn, EventReader, ResMut, SystemLabel};
use networking::client::IncomingReliableServerMessage;

use crate::{
    net::InventoryServerMessage,
    server::inventory::{Inventory, SlotItem},
};

pub(crate) fn client_item_added_to_slot(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut inventory: ResMut<Inventory>,
) {
    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(event) => {
                match inventory.slots.get_mut(&event.slot_id) {
                    Some(slot) => {
                        slot.items.push(SlotItem {
                            entity: event.item_entity,
                            position: event.position,
                        });
                    }
                    None => {
                        warn!("couldnt find slot to add to.");
                    }
                }
            }
            _ => (),
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum ClientBuildInventoryLabel {
    AddSlot,
}

pub fn set_active_item(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut inventory: ResMut<Inventory>,
) {
    for event in net.iter() {
        match event.message {
            InventoryServerMessage::SetActiveItem(entity) => {
                inventory.active_item = Some(entity);
            }
            _ => (),
        }
    }
}
