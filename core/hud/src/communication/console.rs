use bevy::prelude::{EventReader, EventWriter};
use console_commands::net::{ClientConsoleInput, ConsoleCommandsClientMessage};
use networking::client::OutgoingReliableClientMessage;

pub(crate) fn console_input(
    mut events: EventReader<ClientConsoleInput>,
    mut net: EventWriter<OutgoingReliableClientMessage<ConsoleCommandsClientMessage>>,
) {
    for input in events.iter() {
        net.send(OutgoingReliableClientMessage {
            message: ConsoleCommandsClientMessage::ConsoleCommand(input.clone()),
        });
    }
}
