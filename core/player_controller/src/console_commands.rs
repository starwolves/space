use api::{
    data::{ConnectedPlayer, HandleToEntity},
    humanoid::UsedNames,
    inventory::Inventory,
};
use bevy::prelude::EventReader;
use bevy::prelude::{Commands, Entity, EventWriter, Local, Query, Res, ResMut, Transform};
use console_commands::commands::{
    NetConsoleCommands, NetEntityConsole, CONSOLE_ERROR_COLOR, CONSOLE_SUCCESS_COLOR,
};
use entity::{meta::EntityDataResource, spawn::DefaultSpawnEvent};
use gridmap::{commands::rcon_spawn_entity, grid::GridmapMain};
use inventory_item::spawn::rcon_spawn_held_entity;
use networking::messages::{GodotVariantValues, InputConsoleCommand, ReliableServerMessage};
use pawn::pawn::Pawn;
use std::collections::HashMap;

/// Perform RCON console commands.
pub(crate) fn rcon_console_commands(
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
                GodotVariantValues::String(value) => {
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
        } else if console_command_event.command_name == "rconStatus"
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

/// Perform entity console commands.
pub(crate) fn entity_console_commands(
    mut queue: EventReader<InputConsoleCommand>,

    mut commands: Commands,
    mut net_console_commands: EventWriter<NetEntityConsole>,
    gridmap_main: Res<GridmapMain>,
    mut used_names: ResMut<UsedNames>,
    mut rigid_body_positions: Query<(&Transform, &Pawn)>,
    connected_players: Query<&ConnectedPlayer>,

    handle_to_entity: Res<HandleToEntity>,
    entity_data: ResMut<EntityDataResource>,
    mut default_spawner: EventWriter<DefaultSpawnEvent>,
) {
    for console_command_event in queue.iter() {
        let player_entity;
        match connected_players.get(console_command_event.entity) {
            Ok(s) => {
                player_entity = s;
            }
            Err(_rr) => {
                continue;
            }
        }

        if player_entity.rcon == false {
            match console_command_event.handle_option {
                Some(t) => {
                    net_console_commands.send(NetEntityConsole {
                        handle: t,
                        message: ReliableServerMessage::ConsoleWriteLine(
                            "[color=#ff6600]RCON status denied.[/color]".to_string(),
                        ),
                    });
                }
                None => {}
            }
            return;
        }

        if console_command_event.command_name == "spawn" {
            let entity_name;

            match &console_command_event.command_arguments[0] {
                GodotVariantValues::String(value) => {
                    entity_name = value;
                }
                _ => {
                    return;
                }
            }

            let spawn_amount;

            match &console_command_event.command_arguments[1] {
                GodotVariantValues::Int(value) => {
                    spawn_amount = *value;
                }
                _ => {
                    return;
                }
            }

            let player_selector;

            match &console_command_event.command_arguments[2] {
                GodotVariantValues::String(value) => {
                    player_selector = value;
                }
                _ => {
                    return;
                }
            }

            rcon_spawn_entity(
                entity_name.to_string(),
                player_selector.to_string(),
                spawn_amount,
                &mut commands,
                console_command_event.entity,
                console_command_event.handle_option,
                &mut rigid_body_positions,
                &mut net_console_commands,
                &gridmap_main,
                &mut used_names,
                &handle_to_entity,
                &entity_data,
                &mut default_spawner,
            );
        }
    }
}

/// Password to gain access to console RCON commands.
const RCON_PASSWORD: &str = "KA-BAR";

/// Resource to protect against RCON password bruteforce.
#[derive(Default)]
pub(crate) struct BruteforceProtection {
    /// Wrong password attempts by handle.
    pub tracking_data: HashMap<u64, u8>,
    /// Blacklisted handles.
    pub blacklist: Vec<u64>,
}

/// Perform RCON authorization.
pub(crate) fn rcon_authorization(
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

/// Manage requests for RCON permission status.
pub(crate) fn rcon_status(
    connected_players: &mut Query<&mut ConnectedPlayer>,
    client_handle: u64,
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

/// Manage inventory item console commands.
pub(crate) fn inventory_item_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut commands: Commands,
    mut net_console_commands: EventWriter<NetEntityConsole>,
    gridmap_main: Res<GridmapMain>,
    mut used_names: ResMut<UsedNames>,
    mut rigid_body_positions: Query<(&Transform, &Pawn)>,
    mut inventory_components: Query<&mut Inventory>,
    connected_players: Query<&ConnectedPlayer>,

    handle_to_entity: Res<HandleToEntity>,
    mut entity_data: ResMut<EntityDataResource>,
    mut default_spawner: EventWriter<DefaultSpawnEvent>,
) {
    for console_command_event in queue.iter() {
        let player_entity;
        match connected_players.get(console_command_event.entity) {
            Ok(s) => {
                player_entity = s;
            }
            Err(_rr) => {
                continue;
            }
        }

        if player_entity.rcon == false {
            match console_command_event.handle_option {
                Some(t) => {
                    net_console_commands.send(NetEntityConsole {
                        handle: t,
                        message: ReliableServerMessage::ConsoleWriteLine(
                            "[color=#ff6600]RCON status denied.[/color]".to_string(),
                        ),
                    });
                }
                None => {}
            }

            return;
        }

        if console_command_event.command_name == "spawnHeld" {
            let entity_name;

            match &console_command_event.command_arguments[0] {
                GodotVariantValues::String(value) => {
                    entity_name = value;
                }
                _ => {
                    return;
                }
            }

            let player_selector;

            match &console_command_event.command_arguments[1] {
                GodotVariantValues::String(value) => {
                    player_selector = value;
                }
                _ => {
                    return;
                }
            }

            rcon_spawn_held_entity(
                entity_name.to_string(),
                player_selector.to_string(),
                &mut commands,
                console_command_event.entity,
                console_command_event.handle_option,
                &mut net_console_commands,
                &mut inventory_components,
                &mut rigid_body_positions,
                &gridmap_main,
                &mut used_names,
                &handle_to_entity,
                &mut entity_data,
                &mut default_spawner,
            );
        }
    }
}
