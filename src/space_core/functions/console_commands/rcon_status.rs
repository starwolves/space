use bevy::prelude::{Entity, EventWriter, Query};

use crate::space_core::{components::connected_player::ConnectedPlayer, events::net::net_console_commands::NetConsoleCommands, resources::network_messages::ReliableServerMessage};

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
                "[color=#3cff00]RCON status granted![/color]"
                .to_string()
            ),
        });

    } else {

        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=#ff6600]RCON status denied.[/color]"
                .to_string()
            ),
        });

    }


}
