use crate::chat::NewChatMessage;
use bevy::prelude::warn;
use networking::server::HandleToEntity;

use bevy::prelude::{EventWriter, Res};

use crate::net::ChatClientMessage;
use networking::server::IncomingReliableClientMessage;

use bevy::prelude::EventReader;
/// Manage incoming network messages from clients.

pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ChatClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_chat_message_event: EventWriter<NewChatMessage>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            ChatClientMessage::InputChatMessage(i_message) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_chat_message_event.send(NewChatMessage {
                            messenger_entity_option: Some(*player_entity),
                            messenger_name_option: None,
                            raw_message: i_message,
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
