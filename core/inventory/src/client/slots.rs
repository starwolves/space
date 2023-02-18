use bevy::prelude::{EventReader, EventWriter, ResMut};
use networking::client::IncomingReliableServerMessage;

use crate::{
    net::InventoryServerMessage,
    server::inventory::{Inventory, Slot},
};

#[derive(Clone)]
pub struct AddedSlot {
    pub slot: Slot,
    pub id: u8,
}

pub(crate) fn client_slot_added(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut inventory: ResMut<Inventory>,
    mut event: EventWriter<AddedSlot>,
) {
    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::AddedSlot(slot) => {
                let index = inventory.slots.len();
                inventory.slots.insert(index as u8, slot.clone());
                event.send(AddedSlot {
                    slot: slot.clone(),
                    id: index as u8,
                });
            }
            _ => (),
        }
    }
}
