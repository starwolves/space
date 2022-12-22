use bevy::prelude::warn;

use crate::commands::InputConsoleCommand;
use bevy::prelude::{EventWriter, Res};
use networking::server::HandleToEntity;

use crate::net::ConsoleCommandsClientMessage;
use bevy::prelude::EventReader;
use networking::server::IncomingReliableClientMessage;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: EventReader<IncomingReliableClientMessage<ConsoleCommandsClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut console_commands_queue: EventWriter<InputConsoleCommand>,
) {
    for message in server.iter() {
        let client_message = message.message.clone();

        match client_message {
            ConsoleCommandsClientMessage::ConsoleCommand(command_name, variant_arguments) => {
                match handle_to_entity.map.get(&message.handle) {
                    Some(player_entity) => {
                        console_commands_queue.send(InputConsoleCommand {
                            handle_option: Some(message.handle),
                            entity: *player_entity,
                            command_name: command_name,
                            command_arguments: variant_arguments,
                        });
                    }
                    None => {
                        warn!("Couldn't find player_entity belonging to console_command sender handle.");
                    }
                }
            }
        }
    }
}
