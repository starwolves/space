use std::collections::HashMap;

use bevy::prelude::EventWriter;
use networking::server::TextTreeBit;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum UiClientMessage {
    TextTreeInput(Option<u64>, String, String, String),
}
use bevy::prelude::EventReader;
use networking::typenames::IncomingReliableClientMessage;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<UiClientMessage>>,
    mut text_tree_input_selection: EventWriter<TextTreeInputSelection>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            UiClientMessage::TextTreeInput(
                belonging_entity,
                tab_action_id,
                menu_id,
                input_selection,
            ) => {
                text_tree_input_selection.send(TextTreeInputSelection {
                    handle: message.handle,
                    menu_id,
                    menu_selection: input_selection,
                    belonging_entity,
                    action_id: tab_action_id,
                });
            }
        }
    }
}
/// Client text tree input selection event.
#[cfg(feature = "server")]
pub struct TextTreeInputSelection {
    /// Handle of the submitter of the selection.
    pub handle: u64,
    /// Menu ID.
    pub menu_id: String,
    /// The selection on the menu.
    pub menu_selection: String,
    /// The action ID.
    pub action_id: String,
    /// The entity submitting the selection.
    pub belonging_entity: Option<u64>,
}
/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum UiServerMessage {
    TextTreeSelection(
        Option<u64>,
        String,
        String,
        String,
        HashMap<String, TextTreeBit>,
    ),
    UIAddNotice(String),
    UIRemoveNotice(String),
    UIRequestInput(String),
}
