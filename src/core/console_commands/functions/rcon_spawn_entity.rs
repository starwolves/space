use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    system::{Commands, Query, Res, ResMut},
};
use bevy_log::warn;
use bevy_transform::prelude::Transform;

use crate::core::{
    connected_player::{
        functions::player_selector_to_entities::player_selector_to_entities,
        resources::HandleToEntity,
    },
    console_commands::events::NetConsoleCommands,
    entity::{
        functions::spawn_entity::spawn_entity, resources::EntityDataResource,
        spawn::DefaultSpawnEvent,
    },
    gridmap::resources::GridmapMain,
    networking::resources::ReliableServerMessage,
    pawn::{
        components::Pawn,
        functions::{
            entity_spawn_position_for_player::entity_spawn_position_for_player, CONSOLE_ERROR_COLOR,
        },
        resources::UsedNames,
    },
};

pub fn rcon_spawn_entity(
    entity_name: String,
    target_selector: String,
    mut spawn_amount: i64,
    commands: &mut Commands,
    command_executor_entity: Entity,
    command_executor_handle_option: Option<u32>,
    rigid_body_positions: &mut Query<(&Transform, &Pawn)>,
    net_console_commands: &mut EventWriter<NetConsoleCommands>,
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
                net_console_commands.send(NetConsoleCommands {
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
                player_handle = *handle;
            }
            None => {
                warn!(
                    "spawn_entity console command couldn't find handle belonging to target entity."
                );
                match command_executor_handle_option {
                    Some(t) => {
                        net_console_commands.send(NetConsoleCommands {
                            handle: t,
                            message: ReliableServerMessage::ConsoleWriteLine(
                                "[color=".to_string() + CONSOLE_ERROR_COLOR + "]An error occured when executing your command, please report this.[/color]"
                            ),
                        });
                    }
                    None => {}
                }

                continue;
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
                        net_console_commands.send(NetConsoleCommands {
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

        if spawn_amount == 1 {
            net_console_commands.send(NetConsoleCommands {
                handle: player_handle,
                message: ReliableServerMessage::ChatMessage(
                    "A new entity has appeared in your proximity.".to_string(),
                ),
            });
        } else if spawn_amount > 1 {
            net_console_commands.send(NetConsoleCommands {
                handle: player_handle,
                message: ReliableServerMessage::ChatMessage(
                    "New entities have appeared in your proximity.".to_string(),
                ),
            });
        }
    }
}
