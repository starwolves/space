use bevy::prelude::ResMut;

use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::server::ReliableClientMessage;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;

use crate::examine::InputExamineEntity;
use bevy::prelude::Entity;
use bevy::prelude::EventWriter;
use bevy::prelude::Res;
use resources::core::HandleToEntity;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_examine_entity: EventWriter<InputExamineEntity>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<ReliableClientMessage, _> =
                bincode::deserialize(&message);
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
                ReliableClientMessage::ExamineEntity(entity_id) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_examine_entity.send(InputExamineEntity {
                                handle: handle,
                                examine_entity: Entity::from_bits(entity_id),
                                entity: *player_entity,
                                ..Default::default()
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineEntity sender handle.");
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetUnloadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetLoadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
