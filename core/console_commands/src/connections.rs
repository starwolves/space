use crate::commands::AllConsoleCommands;
use crate::net::ConsoleCommandsServerMessage;
use bevy::prelude::{EventReader, Res};
use networking::server::OutgoingReliableServerMessage;
use player::connections::SendServerConfiguration;

use bevy::prelude::EventWriter;
#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    console_commands: Res<AllConsoleCommands>,
) {
    for event in config_events.iter() {
        let console_commands = console_commands.list.clone();

        server.send(OutgoingReliableServerMessage {
            handle: event.handle,
            message: ConsoleCommandsServerMessage::ConfigConsoleCommands(console_commands),
        });
    }
}
