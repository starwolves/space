use bevy::prelude::ResMut;

use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::server::ReliableClientMessage;

use bevy::prelude::EventWriter;
use networking::server::TextTreeInputSelection;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    mut text_tree_input_selection: EventWriter<TextTreeInputSelection>,
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
                ReliableClientMessage::TextTreeInput(
                    belonging_entity,
                    tab_action_id,
                    menu_id,
                    input_selection,
                ) => {
                    text_tree_input_selection.send(TextTreeInputSelection {
                        handle: handle,
                        menu_id,
                        menu_selection: input_selection,
                        belonging_entity,
                        action_id: tab_action_id,
                    });
                }
                _ => (),
            }
        }
    }
}
