use bevy::prelude::{Commands, EventReader, EventWriter, Local, Query, Res, ResMut, warn};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::{components::{connected_player::ConnectedPlayer, inventory::Inventory, pawn::Pawn}, events::{general::console_command::ConsoleCommand, net::net_console_commands::NetConsoleCommands}, functions::console_commands::{rcon_authorization::{BruteforceProtection, rcon_authorization}, rcon_spawn_entity::rcon_spawn_entity, rcon_spawn_held_entity::rcon_spawn_held_entity, rcon_status::rcon_status}, resources::{gridmap_main::GridmapMain, network_messages::ReliableServerMessage, used_names::UsedNames}};

pub fn console_commands(
    mut console_commands_events : EventReader<ConsoleCommand>,
    mut rcon_bruteforce_protection : Local<BruteforceProtection>,
    mut connected_players : Query<&mut ConnectedPlayer>,
    mut rigid_body_positions : Query<(&RigidBodyPosition, &Pawn)>,
    mut inventory_components : Query<&mut Inventory>,

    mut net_console_commands : EventWriter<NetConsoleCommands>,
    mut commands : Commands,

    gridmap_main : Res<GridmapMain>,
    mut used_names : ResMut<UsedNames>,

) {

    for console_command_event in console_commands_events.iter() {

        if console_command_event.command_name == "rcon" {

            match &console_command_event.command_arguments[0] {
                crate::space_core::resources::network_messages::ConsoleCommandVariantValues::String(value) => {
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

        if console_command_event.command_name == "spawn_entity" {

            let entity_name;

            match &console_command_event.command_arguments[0] {
                crate::space_core::resources::network_messages::ConsoleCommandVariantValues::String(value) => {
                    entity_name = value;
                },
                _=> {
                    return;
                },
            }


            match &console_command_event.command_arguments[1] {
                crate::space_core::resources::network_messages::ConsoleCommandVariantValues::Int(value) => {
                    rcon_spawn_entity(
                        entity_name.to_string(),
                        *value,
                        &mut commands,
                        console_command_event.entity,
                        console_command_event.handle,
                        &mut rigid_body_positions,
                        &mut net_console_commands,
                        &gridmap_main,
                        &mut used_names
                    );
                },
                _=> {
                    return;
                },
            }

        } else if console_command_event.command_name == "spawn_held_entity" {

            let entity_name;

            match &console_command_event.command_arguments[0] {
                crate::space_core::resources::network_messages::ConsoleCommandVariantValues::String(value) => {
                    entity_name = value;
                },
                _=> {
                    return;
                },
            }

            match inventory_components.get_mut(console_command_event.entity) {
                Ok(mut inventory) => {
                    rcon_spawn_held_entity(
                        entity_name.to_string(),
                        &mut commands,
                        console_command_event.entity,
                        console_command_event.handle,
                        &mut net_console_commands,
                        &mut inventory,
                        &mut rigid_body_positions,
                        &gridmap_main,
                        &mut used_names
                    );
                },
                Err(_rr) => {warn!("Couldnt find inventory component of player who executed spawn_held_entity command.");return;},
            }

        }

    }

}
