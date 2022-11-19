use bevy::prelude::EventReader;
use console_commands::commands::InputConsoleCommand;

use bevy::prelude::{Commands, EventWriter, Res};

use bevy::prelude::{Query, ResMut, Transform};
use entity::{meta::EntityDataResource, spawn::DefaultSpawnEvent};
use gridmap::grid::GridmapMain;
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::{ConnectedPlayer, HandleToEntity};
use networking::server::{GodotVariantValues, ReliableServerMessage};
use networking_macros::NetMessage;
use pawn::pawn::Pawn;
use player::names::UsedNames;

#[cfg(feature = "server")]
#[derive(NetMessage)]
pub(crate) struct NetBasicConsoleCommands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

/// Perform entity console commands.
#[cfg(feature = "server")]
pub(crate) fn entity_console_commands(
    mut queue: EventReader<InputConsoleCommand>,

    mut commands: Commands,
    mut net_console_commands: EventWriter<NetBasicConsoleCommands>,
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
                    net_console_commands.send(NetBasicConsoleCommands {
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
use crate::player_selectors::player_selector_to_entities;
use bevy::prelude::Entity;

use entity::spawn::spawn_entity;
use gridmap::get_spawn_position::entity_spawn_position_for_player;
use text_api::core::CONSOLE_ERROR_COLOR;
/// Process spawning entities via RCON command as a function. Such as commands for spawning entities.
#[cfg(feature = "server")]
pub(crate) fn rcon_spawn_entity(
    entity_name: String,
    target_selector: String,
    mut spawn_amount: i64,
    commands: &mut Commands,
    command_executor_entity: Entity,
    command_executor_handle_option: Option<u64>,
    rigid_body_positions: &mut Query<(&Transform, &Pawn)>,
    net_console_commands: &mut EventWriter<NetBasicConsoleCommands>,
    gridmap_main: &Res<GridmapMain>,
    used_names: &mut ResMut<UsedNames>,
    handle_to_entity: &Res<HandleToEntity>,
    entity_data: &ResMut<EntityDataResource>,
    default_spawner: &mut EventWriter<DefaultSpawnEvent>,
) {
    if spawn_amount > 5 {
        spawn_amount = 5;
        match command_executor_handle_option {
            Some(t) => {
                net_console_commands.send(NetBasicConsoleCommands {
                    handle: t,
                    message: ReliableServerMessage::ConsoleWriteLine(
                        "Capped amount to 5, maniac protection.".to_string(),
                    ),
                });
            }
            None => {}
        }
    }

    for target_entity in player_selector_to_entities(
        command_executor_entity,
        command_executor_handle_option,
        &target_selector,
        used_names,
        net_console_commands,
    )
    .iter()
    {
        let player_position;
        let standard_character_component;

        match rigid_body_positions.get(*target_entity) {
            Ok((position, pawn_component)) => {
                player_position = position.clone();
                standard_character_component = pawn_component;
            }
            Err(_rr) => {
                continue;
            }
        }

        let player_handle;

        match handle_to_entity.inv_map.get(target_entity) {
            Some(handle) => {
                player_handle = Some(*handle);
            }
            None => {
                player_handle = None;
            }
        }

        let spawn_position = entity_spawn_position_for_player(
            player_position,
            Some(&standard_character_component.facing_direction),
            None,
            gridmap_main,
        );

        let mut final_result = None;

        let mut individual_transform = spawn_position.0.clone();

        for _i in 0..spawn_amount {
            final_result = spawn_entity(
                entity_name.clone(),
                individual_transform,
                commands,
                true,
                entity_data,
                None,
                None,
                None,
                default_spawner,
            );
            individual_transform.translation.x += 0.5;
            individual_transform = entity_spawn_position_for_player(
                individual_transform,
                Some(&standard_character_component.facing_direction),
                None,
                gridmap_main,
            )
            .0;
        }

        if spawn_amount > 0 {
            match final_result {
                Some(_) => {}
                None => match command_executor_handle_option {
                    Some(t) => {
                        net_console_commands.send(NetBasicConsoleCommands {
                            handle: t,
                            message: ReliableServerMessage::ConsoleWriteLine(
                                "[color=".to_string()
                                    + CONSOLE_ERROR_COLOR
                                    + "]Unknown entity name \""
                                    + &entity_name
                                    + "\" was provided.[/color]",
                            ),
                        });
                    }
                    None => {}
                },
            }
        }

        if player_handle.is_some() {
            if spawn_amount == 1 {
                net_console_commands.send(NetBasicConsoleCommands {
                    handle: player_handle.unwrap(),
                    message: ReliableServerMessage::ChatMessage(
                        "A new entity has appeared in your proximity.".to_string(),
                    ),
                });
            } else if spawn_amount > 1 {
                net_console_commands.send(NetBasicConsoleCommands {
                    handle: player_handle.unwrap(),
                    message: ReliableServerMessage::ChatMessage(
                        "New entities have appeared in your proximity.".to_string(),
                    ),
                });
            }
        }
    }
}
use inventory_item::spawn::spawn_held_entity;

use bevy::prelude::warn;
/// Function to spawn an entity in another entity's inventory through an RCON command.
#[cfg(feature = "server")]
pub(crate) fn rcon_spawn_held_entity(
    entity_name: String,
    target_selector: String,
    mut commands: &mut Commands,
    command_executor_entity: Entity,
    command_executor_handle_option: Option<u64>,
    mut net_console_commands: &mut EventWriter<NetBasicConsoleCommands>,
    player_inventory_query: &mut Query<&mut Inventory>,
    mut rigid_body_positions: &mut Query<(&Transform, &Pawn)>,
    gridmap_main: &Res<GridmapMain>,
    mut used_names: &mut ResMut<UsedNames>,
    handle_to_entity: &Res<HandleToEntity>,
    entity_data: &mut ResMut<EntityDataResource>,
    default_spawner: &mut EventWriter<DefaultSpawnEvent>,
) {
    for target_entity in player_selector_to_entities(
        command_executor_entity,
        command_executor_handle_option,
        &target_selector,
        used_names,
        net_console_commands,
    )
    .iter()
    {
        let mut player_inventory;

        match player_inventory_query.get_mut(*target_entity) {
            Ok(inventory) => {
                player_inventory = inventory;
            }
            Err(_rr) => {
                match command_executor_handle_option {
                    Some(t) => {
                        net_console_commands.send(NetBasicConsoleCommands {
                            handle: t,
                            message: ReliableServerMessage::ConsoleWriteLine(
                                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]An error occured when executing your command, please report this.[/color]"
                            ),
                        });
                    }
                    None => {}
                }
                warn!("spawn_held_entity console command couldn't find inventory component beloning to player target.");

                continue;
            }
        }

        let player_handle;

        match handle_to_entity.inv_map.get(target_entity) {
            Some(handle) => {
                player_handle = *handle;
            }
            None => {
                match command_executor_handle_option {
                    Some(t) => {
                        net_console_commands.send(NetBasicConsoleCommands {
                            handle: t,
                            message: ReliableServerMessage::ConsoleWriteLine(
                                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]An error occured when executing your command, please report this.[/color]"
                            ),
                        });
                    }
                    None => {}
                }

                warn!("spawn_held_entity console command couldn't find handle belonging to target entity.");
                continue;
            }
        }

        let mut available_slot = None;

        for slot in player_inventory.slots.iter_mut() {
            let is_hand = matches!(slot.slot_name.as_str(), "left_hand" | "right_hand");
            if is_hand && slot.slot_item.is_none() {
                available_slot = Some(slot);
            }
        }

        match available_slot {
            Some(slot) => {
                let entity_option = spawn_held_entity(
                    entity_name.clone(),
                    commands,
                    command_executor_entity,
                    None,
                    &entity_data,
                    default_spawner,
                );

                match entity_option {
                    Some(entity) => {
                        slot.slot_item = Some(entity);

                        net_console_commands.send(NetBasicConsoleCommands {
                            handle: player_handle,
                            message: ReliableServerMessage::PickedUpItem(
                                entity_name.clone(),
                                entity.to_bits(),
                                slot.slot_name.clone(),
                            ),
                        });

                        net_console_commands.send(NetBasicConsoleCommands {
                            handle: player_handle,
                            message: ReliableServerMessage::ChatMessage(
                                "A new entity has appeared in your hand.".to_string(),
                            ),
                        });
                    }
                    None => match command_executor_handle_option {
                        Some(t) => {
                            net_console_commands.send(NetBasicConsoleCommands {
                                handle: t,
                                message: ReliableServerMessage::ConsoleWriteLine(
                                    "[color=".to_string()
                                        + CONSOLE_ERROR_COLOR
                                        + "]Unknown entity name \""
                                        + &entity_name
                                        + "\" was provided.[/color]",
                                ),
                            });
                        }
                        None => {}
                    },
                }
            }
            None => {
                rcon_spawn_entity(
                    entity_name.clone(),
                    target_selector.clone(),
                    1,
                    &mut commands,
                    command_executor_entity,
                    command_executor_handle_option,
                    &mut rigid_body_positions,
                    &mut net_console_commands,
                    &gridmap_main,
                    &mut used_names,
                    handle_to_entity,
                    &entity_data,
                    default_spawner,
                );
            }
        }
    }
}

use inventory_api::core::Inventory;

/// Manage inventory item console commands.
#[cfg(feature = "server")]
pub(crate) fn inventory_item_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut commands: Commands,
    mut net_console_commands: EventWriter<NetBasicConsoleCommands>,
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
                    net_console_commands.send(NetBasicConsoleCommands {
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
use bevy::prelude::Local;

/// Perform RCON console commands.
#[cfg(feature = "server")]
pub(crate) fn rcon_console_commands(
    mut console_commands_events: EventReader<InputConsoleCommand>,
    mut rcon_bruteforce_protection: Local<BruteforceProtection>,
    mut connected_players: Query<&mut ConnectedPlayer>,
    mut net_console_commands: EventWriter<NetBasicConsoleCommands>,
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

/// Perform RCON authorization.
#[cfg(feature = "server")]
pub(crate) fn rcon_authorization(
    bruteforce_protection: &mut Local<BruteforceProtection>,
    connected_players: &mut Query<&mut ConnectedPlayer>,
    client_handle: u64,
    client_entity: Entity,
    net_console_commands: &mut EventWriter<NetBasicConsoleCommands>,
    input_password: String,
) {
    use text_api::core::CONSOLE_SUCCESS_COLOR;

    if bruteforce_protection.blacklist.contains(&client_handle) {
        net_console_commands.send(NetBasicConsoleCommands {
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

        net_console_commands.send(NetBasicConsoleCommands {
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

        net_console_commands.send(NetBasicConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]Wrong password.[/color]",
            ),
        });
    }
}

/// Manage requests for RCON permission status.
#[cfg(feature = "server")]
pub(crate) fn rcon_status(
    connected_players: &mut Query<&mut ConnectedPlayer>,
    client_handle: u64,
    client_entity: Entity,
    net_console_commands: &mut EventWriter<NetBasicConsoleCommands>,
) {
    use text_api::core::CONSOLE_SUCCESS_COLOR;

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
        net_console_commands.send(NetBasicConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_SUCCESS_COLOR + "]RCON status granted![/color]",
            ),
        });
    } else {
        net_console_commands.send(NetBasicConsoleCommands {
            handle: client_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]RCON status denied.[/color]",
            ),
        });
    }
}
/// Resource with the configuration whether new players should be given RCON upon connection.
#[derive(Default)]
#[cfg(feature = "server")]
pub struct GiveAllRCON {
    pub give: bool,
}
/// Password to gain access to console RCON commands.
const RCON_PASSWORD: &str = "KA-BAR";
use std::collections::HashMap;
/// Resource to protect against RCON password bruteforce.
#[derive(Default)]
pub(crate) struct BruteforceProtection {
    /// Wrong password attempts by handle.
    pub tracking_data: HashMap<u64, u8>,
    /// Blacklisted handles.
    pub blacklist: Vec<u64>,
}
