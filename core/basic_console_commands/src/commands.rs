use bevy::prelude::{EventReader, Resource};
use console_commands::commands::InputConsoleCommand;

use bevy::prelude::{Commands, EventWriter, Res};

use bevy::prelude::{Query, ResMut, Transform};
use entity::{meta::EntityDataResource, spawn::DefaultSpawnEvent};
use gridmap::grid::GridmapMain;
use networking::server::GodotVariantValues;
use networking::server::{ConnectedPlayer, HandleToEntity};
use networking::typenames::OutgoingReliableServerMessage;
use pawn::pawn::Pawn;
use player::names::UsedNames;

/// Perform entity console commands.
#[cfg(feature = "server")]
pub(crate) fn rcon_entity_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut server: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    connected_players: Query<&ConnectedPlayer>,
    mut rcon_spawn_event: EventWriter<RconSpawnEntity>,
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
                    server.send(OutgoingReliableServerMessage {
                        handle: t,
                        message: ConsoleCommandsServerMessage::ConsoleWriteLine(
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

            rcon_spawn_event.send(RconSpawnEntity {
                entity_name: entity_name.to_string(),
                target_selector: player_selector.to_string(),
                spawn_amount: spawn_amount,
                command_executor_handle_option: console_command_event.handle_option,
                command_executor_entity: console_command_event.entity,
            });
        }
    }
}
use crate::player_selectors::player_selector_to_entities;
use bevy::prelude::Entity;

use console_commands::networking::ConsoleCommandsServerMessage;
use entity::spawn::spawn_entity;
use gridmap::get_spawn_position::entity_spawn_position_for_player;
use text_api::core::CONSOLE_ERROR_COLOR;

/// Event for spawning in entities with the rcon command.
pub struct RconSpawnEntity {
    pub entity_name: String,
    pub target_selector: String,
    pub spawn_amount: i64,
    pub command_executor_handle_option: Option<u64>,
    pub command_executor_entity: Entity,
}

/// Process spawning entities via RCON command. Such as commands for spawning entities.
#[cfg(feature = "server")]
pub(crate) fn rcon_spawn_entity(
    mut rcon_spawn_events: EventReader<RconSpawnEntity>,
    mut commands: Commands,
    rigid_body_positions: Query<(&Transform, &Pawn)>,
    mut server_1: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    mut server_2: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
    gridmap_main: Res<GridmapMain>,
    mut used_names: ResMut<UsedNames>,
    handle_to_entity: Res<HandleToEntity>,
    entity_data: ResMut<EntityDataResource>,
    mut default_spawner: EventWriter<DefaultSpawnEvent>,
) {
    for event in rcon_spawn_events.iter() {
        let mut spawn_amount = event.spawn_amount;

        if spawn_amount > 5 {
            spawn_amount = 5;
            match event.command_executor_handle_option {
                Some(t) => {
                    server_1.send(OutgoingReliableServerMessage {
                        handle: t,
                        message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                            "Capped amount to 5, maniac protection.".to_string(),
                        ),
                    });
                }
                None => {}
            }
        }

        for target_entity in player_selector_to_entities(
            event.command_executor_entity,
            event.command_executor_handle_option,
            &event.target_selector,
            &mut used_names,
            &mut server_1,
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
                &gridmap_main,
            );

            let mut final_result = None;

            let mut individual_transform = spawn_position.0.clone();

            for _i in 0..spawn_amount {
                final_result = spawn_entity(
                    event.entity_name.clone(),
                    individual_transform,
                    &mut commands,
                    true,
                    &entity_data,
                    None,
                    None,
                    None,
                    &mut default_spawner,
                );
                individual_transform.translation.x += 0.5;
                individual_transform = entity_spawn_position_for_player(
                    individual_transform,
                    Some(&standard_character_component.facing_direction),
                    None,
                    &gridmap_main,
                )
                .0;
            }

            if spawn_amount > 0 {
                match final_result {
                    Some(_) => {}
                    None => match event.command_executor_handle_option {
                        Some(t) => {
                            server_1.send(OutgoingReliableServerMessage {
                                handle: t,
                                message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                                    "[color=".to_string()
                                        + CONSOLE_ERROR_COLOR
                                        + "]Unknown entity name \""
                                        + &event.entity_name
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
                    server_2.send(OutgoingReliableServerMessage {
                        handle: player_handle.unwrap(),
                        message: NetworkingChatServerMessage::ChatMessage(
                            "A new entity has appeared in your proximity.".to_string(),
                        ),
                    });
                } else if spawn_amount > 1 {
                    server_2.send(OutgoingReliableServerMessage {
                        handle: player_handle.unwrap(),
                        message: NetworkingChatServerMessage::ChatMessage(
                            "New entities have appeared in your proximity.".to_string(),
                        ),
                    });
                }
            }
        }
    }
}
use inventory::networking::InventoryServerMessage;

use bevy::prelude::warn;
use networking::server::NetworkingChatServerMessage;

/// Event for spawning in entities with the rcon command.
pub struct RconSpawnHeldEntity {
    pub entity_name: String,
    pub target_selector: String,
    pub command_executor_handle_option: Option<u64>,
    pub command_executor_entity: Entity,
}
use inventory_item::spawn::spawn_held_entity;

/// Function to spawn an entity in another entity's inventory through an RCON command.
#[cfg(feature = "server")]
pub(crate) fn rcon_spawn_held_entity(
    mut commands: Commands,
    mut server: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    mut server1: EventWriter<OutgoingReliableServerMessage<InventoryServerMessage>>,
    mut server2: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
    mut player_inventory_query: Query<&mut Inventory>,
    mut used_names: ResMut<UsedNames>,
    handle_to_entity: Res<HandleToEntity>,
    entity_data: ResMut<EntityDataResource>,
    mut default_spawner: EventWriter<DefaultSpawnEvent>,
    mut spawn_held_entity_event: EventReader<RconSpawnHeldEntity>,
    mut spawn_entity: EventWriter<RconSpawnEntity>,
) {
    for event in spawn_held_entity_event.iter() {
        for target_entity in player_selector_to_entities(
            event.command_executor_entity,
            event.command_executor_handle_option,
            &event.target_selector,
            &mut used_names,
            &mut server,
        )
        .iter()
        {
            let mut player_inventory;

            match player_inventory_query.get_mut(*target_entity) {
                Ok(inventory) => {
                    player_inventory = inventory;
                }
                Err(_rr) => {
                    match event.command_executor_handle_option {
                        Some(t) => {
                            server.send(OutgoingReliableServerMessage {
                                handle: t,
                                message: ConsoleCommandsServerMessage::ConsoleWriteLine(
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
                    match event.command_executor_handle_option {
                        Some(t) => {
                            server.send(OutgoingReliableServerMessage {
                                handle: t,
                                message: ConsoleCommandsServerMessage::ConsoleWriteLine(
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
                        event.entity_name.clone(),
                        &mut commands,
                        event.command_executor_entity,
                        None,
                        &entity_data,
                        &mut default_spawner,
                    );

                    match entity_option {
                        Some(entity) => {
                            slot.slot_item = Some(entity);

                            server1.send(OutgoingReliableServerMessage {
                                handle: player_handle,
                                message: InventoryServerMessage::PickedUpItem(
                                    event.entity_name.clone(),
                                    entity.to_bits(),
                                    slot.slot_name.clone(),
                                ),
                            });

                            server2.send(OutgoingReliableServerMessage {
                                handle: player_handle,
                                message: NetworkingChatServerMessage::ChatMessage(
                                    "A new entity has appeared in your hand.".to_string(),
                                ),
                            });
                        }
                        None => match event.command_executor_handle_option {
                            Some(t) => {
                                server.send(OutgoingReliableServerMessage {
                                    handle: t,
                                    message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                                        "[color=".to_string()
                                            + CONSOLE_ERROR_COLOR
                                            + "]Unknown entity name \""
                                            + &event.entity_name
                                            + "\" was provided.[/color]",
                                    ),
                                });
                            }
                            None => {}
                        },
                    }
                }
                None => {
                    spawn_entity.send(RconSpawnEntity {
                        entity_name: event.entity_name.clone(),
                        target_selector: event.target_selector.clone(),
                        spawn_amount: 1,
                        command_executor_handle_option: event.command_executor_handle_option,
                        command_executor_entity: event.command_executor_entity,
                    });
                }
            }
        }
    }
}

use inventory_api::core::Inventory;

/// Manage inventory item console commands.
#[cfg(feature = "server")]
pub(crate) fn inventory_item_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut server: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    connected_players: Query<&ConnectedPlayer>,
    mut spawn_entity: EventWriter<RconSpawnHeldEntity>,
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
                    server.send(OutgoingReliableServerMessage {
                        handle: t,
                        message: ConsoleCommandsServerMessage::ConsoleWriteLine(
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

            spawn_entity.send(RconSpawnHeldEntity {
                entity_name: entity_name.to_string(),
                target_selector: player_selector.to_string(),
                command_executor_handle_option: console_command_event.handle_option,
                command_executor_entity: console_command_event.entity,
            });
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
    mut server: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
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
                        &mut server,
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
                &mut server,
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
    server: &mut EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    input_password: String,
) {
    if bruteforce_protection.blacklist.contains(&client_handle) {
        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(
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

        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(
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
        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]Wrong password.[/color]",
            ),
        });
    }
}
use text_api::core::CONSOLE_SUCCESS_COLOR;

/// Manage requests for RCON permission status.
#[cfg(feature = "server")]
pub(crate) fn rcon_status(
    connected_players: &mut Query<&mut ConnectedPlayer>,
    client_handle: u64,
    client_entity: Entity,
    server: &mut EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
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
        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_SUCCESS_COLOR + "]RCON status granted![/color]",
            ),
        });
    } else {
        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]RCON status denied.[/color]",
            ),
        });
    }
}
/// Resource with the configuration whether new players should be given RCON upon connection.
#[derive(Default, Resource)]
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
