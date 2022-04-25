use std::collections::HashMap;

use bevy_app::EventWriter;
use bevy_ecs::{
    entity::Entity,
    system::{Local, Query},
};

use crate::space::core::{
    connected_player::components::ConnectedPlayer,
    console_commands::events::NetConsoleCommands,
    networking::resources::ReliableServerMessage,
    pawn::functions::{CONSOLE_ERROR_COLOR, CONSOLE_SUCCESS_COLOR},
};

const RCON_PASSWORD: &str = "KA-BAR";

#[derive(Default)]
pub struct BruteforceProtection {
    pub tracking_data: HashMap<u64, u8>,
    pub blacklist: Vec<u64>,
}

pub fn rcon_authorization(
    bruteforce_protection: &mut Local<BruteforceProtection>,
    connected_players: &mut Query<&mut ConnectedPlayer>,
    client_handle: u64,
    client_entity: Entity,
    net_console_commands: &mut EventWriter<NetConsoleCommands>,
    input_password: String,
) {
    if bruteforce_protection.blacklist.contains(&client_handle) {
        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string()
                    + CONSOLE_ERROR_COLOR
                    + "]Too many past attempts, blacklisted.[/color]",
            ),
        });
        return;
    }

    if input_password == RCON_PASSWORD {
        let mut connected_player_component;

        match connected_players.get_mut(client_entity) {
            Ok(s) => {
                connected_player_component = s;
            }
            Err(_rr) => {
                return;
            }
        }

        connected_player_component.rcon = true;

        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_SUCCESS_COLOR + "]RCON status granted![/color]",
            ),
        });
    } else {
        match bruteforce_protection.tracking_data.get_mut(&client_handle) {
            Some(attempt_amount) => {
                *attempt_amount += 1;
                if attempt_amount > &mut 10 {
                    bruteforce_protection.blacklist.push(client_handle);
                }
            }
            None => {
                bruteforce_protection.tracking_data.insert(client_handle, 1);
            }
        }

        net_console_commands.send(NetConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]Wrong password.[/color]",
            ),
        });
    }
}
