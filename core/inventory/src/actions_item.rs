use actions::core::{Action, ActionData, BuildingActions};
use bevy::prelude::{Query, ResMut};

use crate::item::InventoryItem;

/// Build inventory item actions like pickup.

pub(crate) fn build_actions(
    mut building_action_data: ResMut<BuildingActions>,
    inventory_items: Query<&InventoryItem>,
) {
    for building_action in building_action_data.list.iter_mut() {
        match building_action.target_entity_option {
            Some(examined_entity) => match inventory_items.get(examined_entity) {
                Ok(_) => {
                    let mut new_vec = vec![ActionData {
                        data: Action {
                            id: "actions::inventory/pickup".to_string(),
                            text: "Pickup".to_string(),
                            tab_list_priority: u8::MAX - 1,
                        },
                        approved: None,
                    }];

                    building_action.actions.append(&mut new_vec);
                }
                Err(_rr) => {}
            },
            None => {}
        }
    }
}
use bevy::prelude::{warn, EventWriter};

use crate::net::InventoryClientMessage;

/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<InventoryClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut use_world_item: EventWriter<InputUseWorldItem>,
    mut drop_current_item: EventWriter<InputDropCurrentItem>,
    mut switch_hands: EventWriter<InputSwitchHands>,
    mut wear_items: EventWriter<InputWearItem>,
    mut take_off_item: EventWriter<InputTakeOffItem>,
    mut input_throw_item: EventWriter<InputThrowItem>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            InventoryClientMessage::UseWorldItem(entity_id) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        use_world_item.send(InputUseWorldItem {
                            using_entity: *player_entity,
                            used_entity: Entity::from_bits(entity_id),
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to UseWorldItem sender handle."
                        );
                    }
                }
            }

            InventoryClientMessage::DropCurrentItem(position_option) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        drop_current_item.send(InputDropCurrentItem {
                            pickuper_entity: *player_entity,
                            input_position_option: position_option,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to DropCurrentItem sender handle.");
                    }
                }
            }

            InventoryClientMessage::SwitchHands => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        switch_hands.send(InputSwitchHands {
                            entity: *player_entity,
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to SwitchHands sender handle."
                        );
                    }
                }
            }

            InventoryClientMessage::WearItem(item_id, wear_slot) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        wear_items.send(InputWearItem {
                            wearer_entity: *player_entity,
                            worn_entity_bits: item_id,
                            wear_slot: wear_slot,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to WearItem sender handle.");
                    }
                }
            }

            InventoryClientMessage::TakeOffItem(slot_name) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        take_off_item.send(InputTakeOffItem {
                            entity: *player_entity,
                            slot_name: slot_name,
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to take_off_item sender handle."
                        );
                    }
                }
            }

            InventoryClientMessage::ThrowItem(position, angle) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_throw_item.send(InputThrowItem {
                            entity: *player_entity,
                            position,
                            angle,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to InputThrowItem sender handle.");
                    }
                }
            }
        }
    }
}

use crate::item_events::InputDropCurrentItem;
use crate::item_events::InputTakeOffItem;
use crate::item_events::InputThrowItem;
use crate::item_events::InputUseWorldItem;
use crate::item_events::InputWearItem;
use crate::switch_hands::InputSwitchHands;
use bevy::prelude::Entity;
use bevy::prelude::Res;
use networking::server::HandleToEntity;

use bevy::prelude::EventReader;
use networking::server::IncomingReliableClientMessage;
