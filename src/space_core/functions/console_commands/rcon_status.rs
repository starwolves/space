use bevy::prelude::{Entity, EventWriter, Query};

use crate::space_core::{components::connected_player::ConnectedPlayer, events::net::net_console_commands::NetConsoleCommands, resources::network_messages::ReliableServerMessage};

use super::{CONSOLE_ERROR_COLOR, CONSOLE_SUCCESS_COLOR};

pub fn rcon_status(
    connected_players : &mut Query<&mut ConnectedPlayer>,
    client_handle:u32,
    client_entity : Entity,
    net_console_commands : &mut EventWriter<NetConsoleCommands>,
) {

    let connected_player_component = connected_players.get_mut(client_entity).unwrap();

    if connected_player_component.rcon {

        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_SUCCESS_COLOR + "]RCON status granted![/color]"
            ),
        });

    } else {

        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]RCON status denied.[/color]"
            ),
        });

    }

}
