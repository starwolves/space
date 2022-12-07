use bevy::prelude::warn;
use bevy::prelude::{EventWriter, Res};

use networking::server::HandleToEntity;
use player::names::InputAccountName;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum PawnClientMessage {
    AccountName(String),
}

use bevy::prelude::EventReader;
use networking::typenames::IncomingReliableClientMessage;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<PawnClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_global_name: EventWriter<InputAccountName>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            PawnClientMessage::AccountName(input_name) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        input_global_name.send(InputAccountName {
                            entity: *player_entity,
                            input_name,
                        });
                    }
                    None => {
                        warn!(
                            "Couldn't find player_entity belonging to InputUserName sender handle."
                        );
                    }
                }
            }
        }
    }
}
