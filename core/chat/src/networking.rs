use bevy::prelude::ResMut;
use serde::{Deserialize, Serialize};

use crate::chat::NewChatMessage;
use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::server::HandleToEntity;

use bevy::prelude::{EventWriter, Res};

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ChatClientMessage {
    InputChatMessage(String),
}

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_chat_message_event: EventWriter<NewChatMessage>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<ChatClientMessage, _> =
                bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    continue;
                }
            }

            match client_message {
                ChatClientMessage::InputChatMessage(message) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_chat_message_event.send(NewChatMessage {
                                messenger_entity_option: Some(*player_entity),
                                messenger_name_option: None,
                                raw_message: message,
                                exclusive_radio: false,
                                position_option: None,
                                send_entity_update: true,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to SelectBodyPart sender handle.");
                        }
                    }
                }
            }
        }
    }
}
