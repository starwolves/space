use bevy::prelude::ResMut;

use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

use bevy::prelude::{EventWriter, Res};

use networking::server::HandleToEntity;
use player::names::InputAccountName;
use serde::Deserialize;
use serde::Serialize;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum PawnMessage {
    AccountName(String),
}

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_global_name: EventWriter<InputAccountName>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<PawnMessage, _> = bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    warn!("Received invalid client message.");
                    continue;
                }
            }

            match client_message {
                PawnMessage::AccountName(input_name) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_global_name.send(InputAccountName {
                                entity: *player_entity,
                                input_name,
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to InputUserName sender handle.");
                        }
                    }
                }
            }
        }
    }
}
