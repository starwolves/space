
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::inventory::ItemAddedToSlot;
use crate::inventory::Slot;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum InventoryClientMessage {

}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum InventoryServerMessage {
    ItemAddedToSlot(ItemAddedToSlot),
    AddedSlot(Slot)
}
