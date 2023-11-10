use bevy::prelude::Entity;
use resources::grid::TargetCell;
use serde::Deserialize;
use serde::Serialize;

use crate::core::InputAction;
use crate::core::InputListActions;
use bevy::log::warn;
use networking::server::HandleToEntity;

use crate::net::ActionsClientMessage;
use bevy::prelude::{EventWriter, Res};

use bevy::prelude::EventReader;
use networking::server::IncomingReliableClientMessage;

/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ActionsClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut action_data_entity: EventWriter<InputListActions>,
    mut input_action: EventWriter<InputAction>,
) {
    for message in server.read() {
        let client_message = message.message.clone();
        match client_message {
            ActionsClientMessage::RequestTabData(tab_data) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        action_data_entity.send(InputListActions {
                            action_taker: *player_entity,
                            targetted_entity: tab_data.target_entity_option,
                            with_ui: true,
                            action_taker_item: None,
                            targetted_cell: tab_data.target_cell_option,
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to TabDataEntity sender handle."
                        );
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
