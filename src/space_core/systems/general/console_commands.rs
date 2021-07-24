use bevy::prelude::{Commands, EventReader, EventWriter, Local, Query, Res};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::{components::{connected_player::ConnectedPlayer, pawn::Pawn}, events::{general::console_command::ConsoleCommand, net::net_console_commands::NetConsoleCommands}, functions::{rcon_authorization::{BruteforceProtection, rcon_authorization}, rcon_spawn_entity::rcon_spawn_entity, rcon_status::rcon_status}, resources::gridmap_main::GridmapMain, structs::network_messages::ReliableServerMessage};

pub fn console_commands(
    mut console_commands_events : EventReader<ConsoleCommand>,
    mut rcon_bruteforce_protection : Local<BruteforceProtection>,
    mut connected_players : Query<&mut ConnectedPlayer>,
    mut rigid_body_positions : Query<(&RigidBodyPosition, &Pawn)>,

    mut net_console_commands : EventWriter<NetConsoleCommands>,
    mut commands : Commands,

    gridmap_main : Res<GridmapMain>,

) {

    for console_command_event in console_commands_events.iter() {

        if console_command_event.command_name == "rcon" {

            match &console_command_event.command_arguments[0] {
                crate::space_core::structs::network_messages::ConsoleCommandVariantValues::String(value) => {
                    rcon_authorization(
                        &mut rcon_bruteforce_protection,
                        &mut connected_players,
                        console_command_event.handle,
                        console_command_event.entity,
                        &mut net_console_commands,
                        value.to_string(),
                    );
                },
                _=>(),
            }

        } else if console_command_event.command_name == "rcon_status" {

            rcon_status(
                &mut connected_players,
                console_command_event.handle,
                console_command_event.entity,
                &mut net_console_commands,
            );

        }

        if console_command_event.command_name.starts_with("rcon_") {

            let player_entity = connected_players.get_mut(console_command_event.entity).unwrap();

            if player_entity.rcon == false{
                net_console_commands.send(NetConsoleCommands {
                    handle: console_command_event.handle,
                    message: ReliableServerMessage::ConsoleWriteLine(
                        "[color=#ff6600]RCON status denied.[/color]"
                        .to_string()
                    ),
                });
                return;
            }

        }

        if console_command_event.command_name == "rcon_spawn_entity" {

            let entity_name;

            match &console_command_event.command_arguments[0] {
                crate::space_core::structs::network_messages::ConsoleCommandVariantValues::String(value) => {
                    entity_name = value;
                },
                _=> {
                    return;
                },
            }


            match &console_command_event.command_arguments[1] {
                crate::space_core::structs::network_messages::ConsoleCommandVariantValues::Int(value) => {
                    rcon_spawn_entity(
                        entity_name.to_string(),
                        *value,
                        &mut commands,
                        console_command_event.entity,
                        console_command_event.handle,
                        &mut rigid_body_positions,
                        &mut net_console_commands,
                        &gridmap_main,
                    );
                },
                _=> {
                    return;
                },
            }

        }

    }

}
