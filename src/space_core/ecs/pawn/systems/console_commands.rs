use bevy::prelude::{Commands, EventReader, EventWriter, Local, Query, Res, ResMut};
use bevy_rapier3d::prelude::{RigidBodyPositionComponent};

use crate::space_core::{ecs::{pawn::{components::{Pawn, ConnectedPlayer}, events::{InputConsoleCommand, NetConsoleCommands}, resources::{HandleToEntity, UsedNames}, functions::{rcon_authorization::{BruteforceProtection, rcon_authorization}, rcon_status::rcon_status, rcon_spawn_entity::rcon_spawn_entity, rcon_spawn_held_entity::rcon_spawn_held_entity}}, inventory::components::Inventory, gridmap::resources::GridmapMain, entity::resources::EntityDataResource, networking::resources::{ConsoleCommandVariantValues, ReliableServerMessage, ConsoleCommandVariant}}};

pub fn console_commands(
    mut console_commands_events : EventReader<InputConsoleCommand>,
    mut rcon_bruteforce_protection : Local<BruteforceProtection>,
    mut connected_players : Query<&mut ConnectedPlayer>,
    mut rigid_body_positions : Query<(&RigidBodyPositionComponent, &Pawn)>,
    mut inventory_components : Query<&mut Inventory>,

    mut net_console_commands : EventWriter<NetConsoleCommands>,
    mut commands : Commands,

    gridmap_main : Res<GridmapMain>,
    mut used_names : ResMut<UsedNames>,
    handle_to_entity : Res<HandleToEntity>,
    mut entity_data : ResMut<EntityDataResource>,

) {

    for console_command_event in console_commands_events.iter() {

        if console_command_event.command_name == "rcon" {

            match &console_command_event.command_arguments[0] {
                ConsoleCommandVariantValues::String(value) => {
                    rcon_authorization(
                        &mut rcon_bruteforce_protection,
                        &mut connected_players,
                        console_command_event.handle,
                        console_command_event.entity,
                        &mut net_console_commands,
                        value.to_string(),
                    );
                    return;
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
            return;

        }

        let player_entity;
        match connected_players.get_mut(console_command_event.entity) {
            Ok(s) => {
                player_entity = s;
            },
            Err(_rr) => {
                continue;
            },
        }

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
                ConsoleCommandVariantValues::String(value) => {
                    entity_name = value;
                },
                _=> {
                    return;
                },
            }

            let spawn_amount;

            match &console_command_event.command_arguments[1] {
                ConsoleCommandVariantValues::Int(value) => {
                    spawn_amount = *value;
                },
                _=> {
                    return;
                },
            }

            let player_selector;

            match &console_command_event.command_arguments[2] {
                ConsoleCommandVariantValues::String(value) => {
                    player_selector = value;
                },
                _=> {
                    return;
                },
            }

            rcon_spawn_entity(
                entity_name.to_string(),
                player_selector.to_string(),
                spawn_amount,
                &mut commands,
                console_command_event.entity,
                console_command_event.handle,
                &mut rigid_body_positions,
                &mut net_console_commands,
                &gridmap_main,
                &mut used_names,
                &handle_to_entity,
                &entity_data
            );

        } else if console_command_event.command_name == "spawn_held_entity" {

            let entity_name;

            match &console_command_event.command_arguments[0] {
                ConsoleCommandVariantValues::String(value) => {
                    entity_name = value;
                },
                _=> {
                    return;
                },
            }

            let player_selector;

            match &console_command_event.command_arguments[1] {
                ConsoleCommandVariantValues::String(value) => {
                    player_selector = value;
                },
                _=> {
                    return;
                },
            }

            rcon_spawn_held_entity(
                entity_name.to_string(),
                player_selector.to_string(),
                &mut commands,
                console_command_event.entity,
                console_command_event.handle,
                &mut net_console_commands,
                &mut inventory_components,
                &mut rigid_body_positions,
                &gridmap_main,
                &mut used_names,
                &handle_to_entity,
                &mut entity_data
            );

        }

    }

}


pub fn get_console_commands() -> Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)> {

    vec![
        (
            "rcon".to_string(),
            "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands".to_string(),
            vec![
                (   
                    "password".to_string(),
                    ConsoleCommandVariant::String
                ),
            ]
        ),
        (
            "rcon_status".to_string(),
            "For server administrators only. Check if the server has granted you the RCON status.".to_string(),
            vec![]
        ),
        (
            "spawn_entity".to_string(),
            "For server administrators only. Spawn in entities in proximity.".to_string(),
            vec![
                (
                    "entity_name".to_string(),
                    ConsoleCommandVariant::String
                ),
                (
                    "amount".to_string(),
                    ConsoleCommandVariant::Int
                ),
                (
                    "player_selector".to_string(),
                    ConsoleCommandVariant::String
                ),
            ]
        ),
        (
            "spawn_held_entity".to_string(),
            "For server administrators only. Spawn in held entities in hands or in proximity.".to_string(),
            vec![
                (
                    "entity_name".to_string(),
                    ConsoleCommandVariant::String
                ),
                (
                    "player_selector".to_string(),
                    ConsoleCommandVariant::String
                ),
            ]
        )
    ]

}
