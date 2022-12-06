use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::chat::NewChatMessage;
use bevy::prelude::warn;
use networking::server::HandleToEntity;

use bevy::prelude::{EventWriter, Res};

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ChatClientMessage {
    InputChatMessage(String),
}
use networking::typenames::get_reliable_message;
use networking::typenames::Typenames;

use bevy::prelude::EventReader;
use networking::typenames::IncomingReliableClientMessage;
/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_chat_message_event: EventWriter<NewChatMessage>,
    typenames: Res<Typenames>,
) {
    for message in server.iter() {
        let client_message;
        match get_reliable_message::<ChatClientMessage>(
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
