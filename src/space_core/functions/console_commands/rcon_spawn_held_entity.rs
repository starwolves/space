use bevy::prelude::{Commands, Entity, EventWriter, Query, Res, ResMut, warn};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::{components::{inventory::Inventory, pawn::Pawn}, events::net::net_console_commands::NetConsoleCommands, functions::entity::spawn_entity::spawn_held_entity, resources::{gridmap_main::GridmapMain, handle_to_entity::HandleToEntity, network_messages::ReliableServerMessage, used_names::UsedNames}};

use super::{player_selector_to_entities::player_selector_to_entities, rcon_spawn_entity::rcon_spawn_entity};

pub fn rcon_spawn_held_entity(
    entity_name : String,
    target_selector : String,
    mut commands : &mut Commands,
    command_executor_entity : Entity,
    command_executor_handle : u32,
    mut net_console_commands : &mut EventWriter<NetConsoleCommands>,
    player_inventory_query : &mut Query<&mut Inventory>,
    mut rigid_body_positions : &mut Query<(&RigidBodyPosition, &Pawn)>,
    gridmap_main : &Res<GridmapMain>,
    mut used_names : &mut ResMut<UsedNames>,
    handle_to_entity : &Res<HandleToEntity>,
) {

    for target_entity in player_selector_to_entities(command_executor_entity, command_executor_handle, &target_selector, used_names, net_console_commands).iter() {
        
        let mut player_inventory;

        match player_inventory_query.get_mut(*target_entity) {
            Ok(inventory) => {

                player_inventory = inventory;

            },
            Err(_rr) => {
                warn!("spawn_held_entity console command couldn't find inventory component beloning to player target.");
                net_console_commands.send(NetConsoleCommands {
                    handle: command_executor_handle,
                    message: ReliableServerMessage::ConsoleWriteLine(
                        "[color=#ff6600]An error occured when executing your command, please report this.[/color]"
                        .to_string()
                    ),
                });
                continue;
            },
        }

        let player_handle;

        match handle_to_entity.inv_map.get(target_entity) {
            Some(handle) => {
                player_handle = *handle;
            },
            None => {
                net_console_commands.send(NetConsoleCommands {
                    handle: command_executor_handle,
                    message: ReliableServerMessage::ConsoleWriteLine(
                        "[color=#ff6600]An error occured when executing your command, please report this.[/color]"
                        .to_string()
                    ),
                });
                warn!("spawn_held_entity console command couldn't find handle belonging to target entity.");
                continue;
            },
        }

        let mut available_slot = None;

        for slot in player_inventory.slots.iter_mut() {
    
            if slot.slot_name == "left_hand" && matches!(slot.slot_item, None) {
                available_slot=Some(slot);
            } else if  slot.slot_name == "right_hand" && matches!(slot.slot_item, None) {
                available_slot=Some(slot);
            }
    
        }
    
        match available_slot {
            Some(slot) => {
    
                let entity_option = spawn_held_entity(
                    entity_name.clone(),
                    commands,
                    command_executor_entity,
                    false,
                    None,
                    &mut None,
                );
    
                match entity_option {
                    Some(entity) => {
                        slot.slot_item = Some(entity);
    
                        net_console_commands.send(NetConsoleCommands {
                            handle: player_handle,
                            message: ReliableServerMessage::PickedUpItem(entity_name.clone(), entity.to_bits(), slot.slot_name.clone()),
                        });

                        net_console_commands.send(NetConsoleCommands {
                            handle: player_handle,
                            message: ReliableServerMessage::ChatMessage("A new entity has appeared in your hand.".to_string()),
                        });
                        
                    },
                    None => {
                        net_console_commands.send(NetConsoleCommands {
                            handle: command_executor_handle,
                            message: ReliableServerMessage::ConsoleWriteLine(
                                "[color=#ff6600]Unknown entity name \"".to_string() + &entity_name + " \" was provided.[/color]"
                            ),
                        });
                    },
                }
    
                
    
            },
            None => {
    
                rcon_spawn_entity(
                    entity_name.clone(),
                    target_selector.clone(),
                    1,
                    &mut commands,
                    command_executor_entity,
                    command_executor_handle,
                    &mut rigid_body_positions,
                    &mut net_console_commands,
                    &gridmap_main,
                    &mut used_names,
                    handle_to_entity
                );
    
            },
        }

    }

}
