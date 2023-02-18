use std::collections::HashMap;

use bevy::prelude::{
    warn, Component, Entity, EventReader, EventWriter, Query, ResMut, Resource, SystemLabel,
};
use math::grid::Vec2Int;
use networking::{
    client::IncomingReliableServerMessage,
    server::{ConnectedPlayer, OutgoingReliableServerMessage},
};
use serde::{Deserialize, Serialize};

use crate::{item::InventoryItem, net::InventoryServerMessage};

#[derive(PartialEq, Copy, Clone, Debug, Default, Serialize, Deserialize)]

pub enum SlotType {
    #[default]
    Generic,
    Helmet,
    Jumpsuit,
    Holster,
}

/// An inventory slot, an inventory can contain many of these.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Slot {
    pub name: String,
    pub slot_type: SlotType,
    pub space: HashMap<Vec2Int, Entity>,
    pub items: Vec<SlotItem>,
    // Dividable by two. 16 by 16 max.
    pub size: Vec2Int,
}
/// Event that adds an inventory item entity to an inventory slot.
pub struct AddItemToSlot {
    pub slot_id: u8,
    pub inventory_entity: Entity,
    pub item_entity: Entity,
    pub item_type_id: u16,
}

/// Adds a slot to an inventory.
pub struct AddSlot {
    pub inventory_entity: Entity,
    pub slot: Slot,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SlotItem {
    pub entity: Entity,
    pub position: Vec2Int,
}

/// The inventory component. Client uses it as a resource to keep it replicated.
#[derive(Component, Default, Resource)]

pub struct Inventory {
    pub slots: HashMap<u8, Slot>,
    pub active_item: Option<Entity>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum SpawnItemLabel {
    SpawnHeldItem,
    AddingComponent,
}

/// Event that fires when an item was successfully added to an inventory slot.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemAddedToSlot {
    pub slot_id: u8,
    pub inventory_entity: Entity,
    pub item_entity: Entity,
    pub position: Vec2Int,
    pub item_type_id: u16,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum InventorySlotLabel {
    AddSlotToInventory,
}

pub(crate) fn add_slot_to_inventory(
    mut events: EventReader<AddSlot>,
    mut inventory_query: Query<(&mut Inventory, Option<&ConnectedPlayer>)>,
    mut net: EventWriter<OutgoingReliableServerMessage<InventoryServerMessage>>,
) {
    for event in events.iter() {
        match inventory_query.get_mut(event.inventory_entity) {
            Ok((mut inventory, connection_option)) => {
                let index = inventory.slots.len();
                inventory.slots.insert(index as u8, event.slot.clone());
                match connection_option {
                    Some(c) => {
                        net.send(OutgoingReliableServerMessage {
                            handle: c.handle,
                            message: InventoryServerMessage::AddedSlot(event.slot.clone()),
                        });
                    }
                    None => {}
                }
            }
            Err(_) => {
                warn!("Couldnt find inventory holder.");
            }
        }
    }
}

pub(crate) fn add_item_to_slot(
    mut added: EventWriter<ItemAddedToSlot>,
    mut events: EventReader<AddItemToSlot>,
    mut inventory_query: Query<&mut Inventory>,
    inventory_item_query: Query<&InventoryItem>,
) {
    for event in events.iter() {
        match inventory_item_query.get(event.item_entity) {
            Ok(inventory_item_component) => match inventory_query.get_mut(event.inventory_entity) {
                Ok(mut inventory_component) => {
                    // inventory_component.slots is still zero ;(
                    for (id, slot) in inventory_component.slots.iter_mut() {
                        if id == &event.slot_id {
                            let start_x = -(slot.size.x / 2);
                            let start_y = slot.size.y / 2;

                            let start_position = Vec2Int {
                                x: start_x,
                                y: start_y,
                            };
                            let mut slot_start_position = start_position.clone();
                            let mut free = false;
                            let test_cells_amount = inventory_item_component.slot_size.x
                                * inventory_item_component.slot_size.y;

                            loop {
                                for test_slot_i in 0..test_cells_amount {
                                    let y;
                                    if test_slot_i == 0 {
                                        y = 0;
                                    } else {
                                        y = test_slot_i / inventory_item_component.slot_size.y;
                                    }
                                    let x;
                                    if test_slot_i == 0 {
                                        x = 0;
                                    } else {
                                        x = test_slot_i
                                            - (y * inventory_item_component.slot_size.y);
                                    }

                                    match slot.space.get(&Vec2Int {
                                        x: x + slot_start_position.x,
                                        y: y + slot_start_position.y,
                                    }) {
                                        Some(_) => {
                                            break;
                                        }
                                        None => {}
                                    }

                                    if test_slot_i == test_cells_amount - 1 {
                                        free = true;
                                    }
                                }

                                if free {
                                    break;
                                }

                                if slot_start_position.x
                                    > (slot.size.x / 2) - inventory_item_component.slot_size.x
                                {
                                    slot_start_position.x = start_position.x;
                                    slot_start_position.y -= 1;
                                } else {
                                    slot_start_position.x += 1;
                                }

                                if slot_start_position.y < -(slot.size.y / 2) {
                                    break;
                                }
                            }

                            if !free {
                                warn!("No empty space left in inventory slot.");
                                continue;
                            }

                            slot.items.push(SlotItem {
                                entity: event.item_entity,
                                position: slot_start_position,
                            });

                            for test_slot_i in 0..test_cells_amount {
                                let y;
                                if test_slot_i == 0 {
                                    y = 0;
                                } else {
                                    y = test_slot_i / inventory_item_component.slot_size.y;
                                }
                                let x;
                                if test_slot_i == 0 {
                                    x = 0;
                                } else {
                                    x = test_slot_i - (y * inventory_item_component.slot_size.y);
                                }

                                slot.space.insert(
                                    Vec2Int {
                                        x: x + slot_start_position.x,
                                        y: y + slot_start_position.y,
                                    },
                                    event.item_entity,
                                );
                            }

                            added.send(ItemAddedToSlot {
                                slot_id: event.slot_id,
                                inventory_entity: event.inventory_entity,
                                item_entity: event.item_entity,
                                position: slot_start_position,
                                item_type_id: event.item_type_id,
                            });
                        }
                    }
                }
                Err(_) => {
                    warn!("Couldnt find inventory component for entity");
                }
            },
            Err(_) => {
                warn!("Couldnt find inventory item.");
            }
        }
    }
}

pub(crate) fn added_item_to_slot(
    mut events: EventReader<ItemAddedToSlot>,
    connected_players: Query<&ConnectedPlayer>,
    mut net: EventWriter<OutgoingReliableServerMessage<InventoryServerMessage>>,
) {
    for event in events.iter() {
        match connected_players.get(event.inventory_entity) {
            Ok(player) => {
                net.send(OutgoingReliableServerMessage {
                    handle: player.handle,
                    message: InventoryServerMessage::ItemAddedToSlot(event.clone()),
                });
            }
            Err(_) => {}
        }
    }
}

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
