use bevy::prelude::{EventReader, Resource};
use console_commands::commands::InputConsoleCommand;

use bevy::prelude::{Commands, EventWriter, Res};

use bevy::prelude::{Query, ResMut, Transform};
use console_commands::net::{ConsoleCommandsServerMessage, ConsoleLine};
use gridmap::grid::Gridmap;
use networking::server::OutgoingReliableServerMessage;
use networking::server::{ConnectedPlayer, HandleToEntity};
use pawn::pawn::Pawn;
use player::names::UsedNames;
use ui::fonts::{Fonts, SOURCECODE_REGULAR_FONT};
use ui::text::{
    NetTextSection, COMMUNICATION_FONT_SIZE, CONSOLE_ERROR_COLOR, CONSOLE_SUCCESS_COLOR,
};

/// Perform entity console commands.

pub fn rcon_entity_console_commands<T: EntityType + Default + Send + Sync + 'static>(
    mut queue: EventReader<InputConsoleCommand>,
    mut server: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    connected_players: Query<&ConnectedPlayer>,
    mut rcon_spawn_event: EventWriter<RconSpawnEntity<T>>,
    fonts: Res<Fonts>,
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

        if console_command_event.input.command == "spawn"
            && console_command_event.input.command.len() == 3
        {
            let entity_name = console_command_event.input.args[0].clone();

            if entity_name != T::default().get_identity() {
                continue;
            }

            if player_entity.rcon == false {
                match console_command_event.handle_option {
                    Some(t) => {
                        let section = NetTextSection {
                            text: "RCON status denied.".to_string(),
                            font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
                            font_size: COMMUNICATION_FONT_SIZE,
                            color: CONSOLE_ERROR_COLOR,
                        };

                        server.send(OutgoingReliableServerMessage {
                            handle: t,
                            message: ConsoleCommandsServerMessage::ConsoleWriteLine(ConsoleLine {
                                sections: vec![section],
                            }),
                        });
                    }
                    None => {}
                }
                return;
            }
            let spawn_amount;
            match console_command_event.input.args[1].parse::<i64>() {
                Ok(t) => {
                    spawn_amount = t;
                }
                Err(_) => {
                    continue;
                }
            };

            let player_selector = console_command_event.input.args[2].clone();

            rcon_spawn_event.send(RconSpawnEntity {
                entity_type: T::default(),
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

use entity::spawn::spawn_entity;
use gridmap::get_spawn_position::entity_spawn_position_for_player;

/// Event for spawning in entities with the rcon command.
pub struct RconSpawnEntity<T: EntityType + Send + Sync + 'static> {
    pub entity_type: T,
    pub target_selector: String,
    pub spawn_amount: i64,
    pub command_executor_handle_option: Option<u64>,
    pub command_executor_entity: Entity,
}
use entity::entity_types::EntityType;
use entity::spawn::SpawnEntity;

/// Process spawning entities via RCON command. Such as commands for spawning entities.

pub fn rcon_spawn_entity<T: EntityType + Clone + Send + Sync + 'static>(
    mut rcon_spawn_events: EventReader<RconSpawnEntity<T>>,
    mut commands: Commands,
    rigid_body_positions: Query<(&Transform, &Pawn)>,
    mut server_1: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    mut server_2: EventWriter<OutgoingReliableServerMessage<NetworkingChatServerMessage>>,
    gridmap_main: Res<Gridmap>,
    mut used_names: ResMut<UsedNames>,
    handle_to_entity: Res<HandleToEntity>,
    mut default_spawner: EventWriter<SpawnEntity<T>>,
    fonts: Res<Fonts>,
) {
    for event in rcon_spawn_events.iter() {
        let mut spawn_amount = event.spawn_amount;

        if spawn_amount > 5 {
            spawn_amount = 5;
            match event.command_executor_handle_option {
                Some(t) => {
                    let section = NetTextSection {
                        text: "RCON status denied.".to_string(),
                        font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
                        font_size: COMMUNICATION_FONT_SIZE,
                        color: CONSOLE_ERROR_COLOR,
                    };

                    server_1.send(OutgoingReliableServerMessage {
                        handle: t,
                        message: ConsoleCommandsServerMessage::ConsoleWriteLine(
                            // "Capped amount to 5, maniac protection.".to_string(),
                            ConsoleLine {
                                sections: vec![section],
                            },
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
            &fonts,
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

            let mut individual_transform = spawn_position.0.clone();

            for _i in 0..spawn_amount {
                spawn_entity(
                    event.entity_type.clone(),
                    individual_transform,
                    &mut commands,
                    true,
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

use networking::server::NetworkingChatServerMessage;

/// Event for spawning in entities with the rcon command.
pub struct RconSpawnHeldEntity<T> {
    pub entity_type: T,
    pub target_selector: String,
    pub command_executor_handle_option: Option<u64>,
    pub command_executor_entity: Entity,
}

use bevy::prelude::Local;

/// Perform RCON console commands.

pub(crate) fn rcon_console_commands(
    mut console_commands_events: EventReader<InputConsoleCommand>,
    mut rcon_bruteforce_protection: Local<BruteforceProtection>,
    mut connected_players: Query<&mut ConnectedPlayer>,
    mut server: EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    fonts: Res<Fonts>,
) {
    for console_command_event in console_commands_events.iter() {
        if console_command_event.input.command == "rcon"
            && console_command_event.input.args.len() == 1
            && console_command_event.handle_option.is_some()
            && !console_command_event.input.args.is_empty()
        {
            rcon_authorization(
                &mut rcon_bruteforce_protection,
                &mut connected_players,
                console_command_event.handle_option.unwrap(),
                console_command_event.entity,
                &mut server,
                console_command_event.input.args[0].to_string(),
                &fonts,
            );
        } else if console_command_event.input.command == "rconStatus"
            && console_command_event.handle_option.is_some()
        {
            rcon_status(
                &mut connected_players,
                console_command_event.handle_option.unwrap(),
                console_command_event.entity,
                &mut server,
                &fonts,
            );
        }
    }
}

/// Perform RCON authorization.

pub(crate) fn rcon_authorization(
    bruteforce_protection: &mut Local<BruteforceProtection>,
    connected_players: &mut Query<&mut ConnectedPlayer>,
    client_handle: u64,
    client_entity: Entity,
    server: &mut EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    input_password: String,
    fonts: &Res<Fonts>,
) {
    if bruteforce_protection.blacklist.contains(&client_handle) {
        let section = NetTextSection {
            text: "Too many past attempts, blacklisted.".to_string(),
            font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
            font_size: COMMUNICATION_FONT_SIZE,
            color: CONSOLE_ERROR_COLOR,
        };

        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(ConsoleLine {
                sections: vec![section],
            }),
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

        let section = NetTextSection {
            text: "RCON status granted!".to_string(),
            font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
            font_size: COMMUNICATION_FONT_SIZE,
            color: CONSOLE_SUCCESS_COLOR,
        };

        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(ConsoleLine {
                sections: vec![section],
            }),
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

        let section = NetTextSection {
            text: "Wrong password.".to_string(),
            font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
            font_size: COMMUNICATION_FONT_SIZE,
            color: CONSOLE_ERROR_COLOR,
        };

        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(ConsoleLine {
                sections: vec![section],
            }),
        });
    }
}

/// Manage requests for RCON permission status.

pub(crate) fn rcon_status(
    connected_players: &mut Query<&mut ConnectedPlayer>,
    client_handle: u64,
    client_entity: Entity,
    server: &mut EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    fonts: &Res<Fonts>,
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
        let section = NetTextSection {
            text: "RCON status granted!".to_string(),
            font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
            font_size: COMMUNICATION_FONT_SIZE,
            color: CONSOLE_SUCCESS_COLOR,
        };

        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(ConsoleLine {
                sections: vec![section],
            }),
        });
    } else {
        let section = NetTextSection {
            text: "RCON status denied.".to_string(),
            font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
            font_size: COMMUNICATION_FONT_SIZE,
            color: CONSOLE_ERROR_COLOR,
        };
        server.send(OutgoingReliableServerMessage {
            handle: client_handle,
            message: ConsoleCommandsServerMessage::ConsoleWriteLine(ConsoleLine {
                sections: vec![section],
            }),
        });
    }
}
/// Resource with the configuration whether new players should be given RCON upon connection.
#[derive(Default, Resource)]

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
