use bevy::prelude::{Entity, EventReader, EventWriter, Local, Query, Res, ResMut};
use bevy_renet::renet::RenetServer;

use crate::core::{
    connected_player::{connection::ConnectedPlayer, plugin::HandleToEntity},
    networking::{
        net::send_net,
        networking::{ConsoleCommandVariant, ConsoleCommandVariantValues, ReliableServerMessage},
        plugin::NetEvent,
    },
};

use super::rcon::{rcon_authorization, rcon_status, BruteforceProtection};

pub const CONSOLE_SUCCESS_COLOR: &str = "#3cff00";
pub const CONSOLE_ERROR_COLOR: &str = "#ff6600";

pub fn console_commands(
    mut console_commands_events: EventReader<InputConsoleCommand>,
    mut rcon_bruteforce_protection: Local<BruteforceProtection>,
    mut connected_players: Query<&mut ConnectedPlayer>,
    mut net_console_commands: EventWriter<NetConsoleCommands>,
) {
    for console_command_event in console_commands_events.iter() {
        if console_command_event.command_name == "rcon"
            && console_command_event.handle_option.is_some()
        {
            match &console_command_event.command_arguments[0] {
                ConsoleCommandVariantValues::String(value) => {
                    rcon_authorization(
                        &mut rcon_bruteforce_protection,
                        &mut connected_players,
                        console_command_event.handle_option.unwrap(),
                        console_command_event.entity,
                        &mut net_console_commands,
                        value.to_string(),
                    );
                }
                _ => (),
            }
        } else if console_command_event.command_name == "rcon_status"
            && console_command_event.handle_option.is_some()
        {
            rcon_status(
                &mut connected_players,
                console_command_event.handle_option.unwrap(),
                console_command_event.entity,
                &mut net_console_commands,
            );
        }
    }
}

pub fn net_system(
    mut net: ResMut<RenetServer>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetConsoleCommands>,
) {
    for new_event in net1.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
}

pub struct NetConsoleCommands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct InputConsoleCommand {
    pub handle_option: Option<u64>,
    pub entity: Entity,
    pub command_name: String,
    pub command_arguments: Vec<ConsoleCommandVariantValues>,
}

#[derive(Default)]
pub struct AllConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)>,
}
