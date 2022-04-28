use bevy_app::{EventReader, EventWriter};
use bevy_ecs::system::{Local, Query, Res, ResMut};
use bevy_networking_turbulence::NetworkResource;

use crate::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    console_commands::functions::{
        rcon_authorization::{rcon_authorization, BruteforceProtection},
        rcon_status::rcon_status,
    },
    networking::{resources::ConsoleCommandVariantValues, send_net, NetEvent},
};

use super::{events::NetConsoleCommands, resources::InputConsoleCommand};

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
    mut net: ResMut<NetworkResource>,
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
