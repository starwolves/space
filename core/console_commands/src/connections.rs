use crate::commands::AllConsoleCommands;
use bevy::prelude::{EventReader, Res};
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use player::connection::SendServerConfiguration;

use bevy::prelude::ResMut;
use bevy_renet::renet::RenetServer;
#[cfg(feature = "server")]
pub(crate) fn configure(
    mut config_events: EventReader<SendServerConfiguration>,
    mut server: ResMut<RenetServer>,
    console_commands: Res<AllConsoleCommands>,
) {
    use crate::networking::ConsoleCommandsServerMessage;

    for event in config_events.iter() {
        let console_commands = console_commands.list.clone();

        server.send_message(
            event.handle,
            RENET_RELIABLE_CHANNEL_ID,
            bincode::serialize(&ConsoleCommandsServerMessage::ConfigConsoleCommands(
                console_commands,
            ))
            .unwrap(),
        );
    }
}
