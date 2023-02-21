use bevy::prelude::Entity;
use serde::Deserialize;
use serde::Serialize;

use crate::core::InputAction;
use crate::core::InputListActionsEntity;
use crate::core::InputListActionsMap;
use crate::core::TargetCell;
use bevy::prelude::warn;
use math::grid::Vec3Int;
use networking::server::HandleToEntity;

use crate::net::ActionsClientMessage;
use bevy::prelude::{EventWriter, Res};

use bevy::prelude::EventReader;
use networking::server::IncomingReliableClientMessage;

/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ActionsClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut action_data_entity: EventWriter<InputListActionsEntity>,
    mut action_data_map: EventWriter<InputListActionsMap>,
    mut input_action: EventWriter<InputAction>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();
        match client_message {
            ActionsClientMessage::TabDataEntity(entity_id_bits) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        action_data_entity.send(InputListActionsEntity {
                            action_taker: *player_entity,
                            targetted_entity: entity_id_bits,
                            with_ui: true,
                            action_taker_item: None,
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to TabDataEntity sender handle."
                        );
                    }
                }
            }

            ActionsClientMessage::TabDataMap(idx, idy, idz, face) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        action_data_map.send(InputListActionsMap {
                            action_taker: *player_entity,
                            gridmap_cell_id: Vec3Int {
                                x: idx,
                                y: idy,
                                z: idz,
                            },
                            with_ui: true,
                            face,
                            action_taker_item: None,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to ExamineMap sender handle.");
                    }
                }
            }

            ActionsClientMessage::TabPressed(tab_pressed) => {
                input_action.send(InputAction {
                    fired_action_id: tab_pressed.id,
                    target_entity_option: tab_pressed.target_entity_option,
                    target_cell_option: tab_pressed.target_cell_option,
                    action_taker: tab_pressed.action_taker,
                    action_taker_item: tab_pressed.action_taker_item,
                });
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct NetAction {
    pub id: String,
    pub text: String,
    pub tab_list_priority: u8,
    pub item_name: String,
    pub action_taker: Entity,
    pub action_taker_item: Option<Entity>,
    pub target_entity_option: Option<Entity>,
    pub target_cell_option: Option<TargetCell>,
}
