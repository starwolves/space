use bevy_internal::prelude::{Entity, EventWriter, Query};

use crate::space::core::{
    networking::resources::ReliableServerMessage,
    pawn::{components::ConnectedPlayer, events::NetConsoleCommands},
};

use super::{CONSOLE_ERROR_COLOR, CONSOLE_SUCCESS_COLOR};

pub fn rcon_status(
    connected_players: &mut Query<&mut ConnectedPlayer>,
    client_handle: u32,
    client_entity: Entity,
    net_console_commands: &mut EventWriter<NetConsoleCommands>,
) {
    let connected_player_component;

    match connected_players.get_mut(client_entity) {
        Ok(s) => {
            connected_player_component = s;
        }
        Err(_rr) => {
            return;
        }
    }

    if connected_player_component.rcon {
        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_SUCCESS_COLOR + "]RCON status granted![/color]",
            ),
        });
    } else {
        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]RCON status denied.[/color]",
            ),
        });
    }
}
