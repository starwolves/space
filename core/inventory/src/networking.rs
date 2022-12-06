use bevy::prelude::warn;
use bevy::prelude::Vec3;

use bevy::prelude::EventWriter;
use bevy::prelude::Res;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use crate::item_events::InputDropCurrentItem;
use crate::item_events::InputTakeOffItem;
use crate::item_events::InputThrowItem;
use crate::item_events::InputUseWorldItem;
use crate::item_events::InputWearItem;
use crate::switch_hands::InputSwitchHands;
use bevy::prelude::Entity;
use networking::server::HandleToEntity;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum InventoryClientMessage {
    UseWorldItem(u64),
    DropCurrentItem(Option<Vec3>),
    SwitchHands,
    WearItem(u64, String),
    TakeOffItem(String),
    ThrowItem(Vec3, f32),
}
use bevy::prelude::EventReader;
use networking::typenames::get_reliable_message;
use networking::typenames::IncomingReliableClientMessage;
use networking::typenames::Typenames;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage>,
    handle_to_entity: Res<HandleToEntity>,
    mut use_world_item: EventWriter<InputUseWorldItem>,
    mut drop_current_item: EventWriter<InputDropCurrentItem>,
    mut switch_hands: EventWriter<InputSwitchHands>,
    mut wear_items: EventWriter<InputWearItem>,
    mut take_off_item: EventWriter<InputTakeOffItem>,
    mut input_throw_item: EventWriter<InputThrowItem>,
    typenames: Res<Typenames>,
) {
    for message in server.iter() {
        let client_message;
        match get_reliable_message::<InventoryClientMessage>(
            &typenames,
            message.message.typename_net,
            &message.message.serialized,
        ) {
            Some(x) => {
                client_message = x;
            }
            None => {
                continue;
            }
        }

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
/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum InventoryServerMessage {
    PickedUpItem(String, u64, String),
    DropItem(String),
    SwitchHands,
    EquippedWornItem(String, u64, String),
}
