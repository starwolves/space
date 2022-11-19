use crate::commands::AllConsoleCommands;
use bevy::prelude::{EventReader, EventWriter, Res};
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::{ReliableServerMessage, ServerConfigMessage};
use networking_macros::NetMessage;
use player::connection::SendServerConfiguration;
#[cfg(feature = "server")]
#[derive(NetMessage)]
pub(crate) struct NetConfigure {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut net_on_new_player_connection: EventWriter<NetConfigure>,
    console_commands: Res<AllConsoleCommands>,
) {
    for event in config_events.iter() {
        let console_commands = console_commands.list.clone();

        net_on_new_player_connection.send(NetConfigure {
            handle: event.handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ConsoleCommands(
                console_commands,
            )),
        });
    }
}
