use bevy_app::EventWriter;
use bevy_ecs::system::{Local, Query, Res, ResMut};

use crate::space::core::{
    connected_player::components::ConnectedPlayer,
    console_commands::functions::{
        rcon_authorization::{rcon_authorization, BruteforceProtection},
        rcon_status::rcon_status,
    },
    networking::resources::ConsoleCommandVariantValues,
};

use super::{events::NetConsoleCommands, resources::QueuedConsoleCommands};

pub fn console_commands(
    console_commands_events: Res<QueuedConsoleCommands>,
    mut rcon_bruteforce_protection: Local<BruteforceProtection>,
    mut connected_players: Query<&mut ConnectedPlayer>,
    mut net_console_commands: EventWriter<NetConsoleCommands>,
) {
    for console_command_event in console_commands_events.queue.iter() {
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

pub fn console_commands_queue_clearer(mut queue: ResMut<QueuedConsoleCommands>) {
    queue.queue.clear();
}
