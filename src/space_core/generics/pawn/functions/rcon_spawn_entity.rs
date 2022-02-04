use bevy::prelude::{Commands, Entity, EventWriter, Query, Res, ResMut, warn};
use bevy_rapier3d::prelude::{RigidBodyPositionComponent};

use crate::space_core::{generics::{pawn::{components::{Pawn, PersistentPlayerData}, events::NetConsoleCommands, resources::{HandleToEntity, UsedNames}}, gridmap::resources::GridmapMain, entity::{resources::EntityDataResource, functions::{isometry_to_transform::isometry_to_transform, spawn_entity::spawn_entity}}, networking::resources::ReliableServerMessage}};

use super::{CONSOLE_ERROR_COLOR, player_selector_to_entities::player_selector_to_entities, entity_spawn_position_for_player::entity_spawn_position_for_player};


pub fn rcon_spawn_entity(
    entity_name : String,
    target_selector : String,
    mut spawn_amount : i64,
    commands : &mut Commands,
    command_executor_entity : Entity,
    command_executor_handle : u32,
    rigid_body_positions : &mut Query<(&RigidBodyPositionComponent, &Pawn)>,
    net_console_commands : &mut EventWriter<NetConsoleCommands>,
    gridmap_main : &Res<GridmapMain>,
    used_names : &mut ResMut<UsedNames>,
    handle_to_entity : &Res<HandleToEntity>,
    entity_data : &ResMut<EntityDataResource>,
) {

    if spawn_amount > 5 {
        spawn_amount = 5;
        net_console_commands.send(NetConsoleCommands {
            handle: command_executor_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "Capped amount to 5, maniac protection."
                .to_string()
            ),
        });
    }

    for target_entity in player_selector_to_entities(command_executor_entity, command_executor_handle, &target_selector, used_names, net_console_commands).iter() {

        let player_position;
        let standard_character_component;
    
        match rigid_body_positions.get(*target_entity) {
            Ok((position, pawn_component)) => {
                player_position = position.position.clone();
                standard_character_component = pawn_component;
            },
            Err(_rr) => {
                continue;
            },
        }

        let player_handle;

        match handle_to_entity.inv_map.get(target_entity) {
            Some(handle) => {
                player_handle = *handle;
            },
            None => {
                warn!("spawn_entity console command couldn't find handle belonging to target entity.");
                net_console_commands.send(NetConsoleCommands {
                    handle: command_executor_handle,
                    message: ReliableServerMessage::ConsoleWriteLine(
                        "[color=".to_string() + CONSOLE_ERROR_COLOR + "]An error occured when executing your command, please report this.[/color]"
                    ),
                });
                continue;
            },
        }
    
        let spawn_position = 
        entity_spawn_position_for_player(
        isometry_to_transform(
        player_position,
            ),
            Some(&standard_character_component.facing_direction),
            None,
            gridmap_main,
        );

        let mut final_result = None;

        let passed_inventory_setup = vec![
            ("jumpsuit".to_string(), "jumpsuitSecurity".to_string()),
            ("helmet".to_string(), "helmetSecurity".to_string()),
        ];

        let persistent_player_data_component = PersistentPlayerData {
            character_name: "".to_string(),
            user_name: "unknownSpawnEntityAssigned".to_string(),
            ..Default::default()
        };
        
        for _i in 0..spawn_amount {
    
            final_result = spawn_entity(
                entity_name.clone(),
                spawn_position.0,
                commands,
                true,
                Some(used_names),
                entity_data,
                None,
                Some((passed_inventory_setup.clone(),persistent_player_data_component.clone())),
            );
    
        }

        if spawn_amount > 0 {

            match final_result {
                Some(_) => {},
                None => {
                    net_console_commands.send(NetConsoleCommands {
                        handle: command_executor_handle,
                        message: ReliableServerMessage::ConsoleWriteLine(
                            "[color=".to_string() + CONSOLE_ERROR_COLOR + "]Unknown entity name \"" + &entity_name + " \" was provided.[/color]"
                        ),
                    });
                },
            }

        }

        if spawn_amount == 1 {
            net_console_commands.send(NetConsoleCommands {
                handle: player_handle,
                message: ReliableServerMessage::ChatMessage("A new entity has appeared in your proximity.".to_string()),
            });
        } else if spawn_amount > 1 {
            net_console_commands.send(NetConsoleCommands {
                handle: player_handle,
                message: ReliableServerMessage::ChatMessage("New entities have appeared in your proximity.".to_string()),
            });
        }
        

    }

}
