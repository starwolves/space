use bevy::prelude::{Commands, Entity, EventWriter, Query, Res, ResMut};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::{components::{inventory::Inventory, pawn::Pawn}, events::net::net_console_commands::NetConsoleCommands, functions::entity::spawn_entity::spawn_held_entity, resources::{gridmap_main::GridmapMain, network_messages::ReliableServerMessage, used_names::UsedNames}};

use super::rcon_spawn_entity::rcon_spawn_entity;

pub fn rcon_spawn_held_entity(
    entity_name : String,
    mut commands : &mut Commands,
    player_entity : Entity,
    player_handle : u32,
    mut net_console_commands : &mut EventWriter<NetConsoleCommands>,
    player_inventory : &mut Inventory,
    mut rigid_body_positions : &mut Query<(&RigidBodyPosition, &Pawn)>,
    gridmap_main : &Res<GridmapMain>,
    mut used_names : &mut ResMut<UsedNames>,
) {

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
                player_entity,
                false,
                None,
                &mut None,
            );

            match entity_option {
                Some(entity) => {
                    slot.slot_item = Some(entity);

                    net_console_commands.send(NetConsoleCommands {
                        handle: player_handle,
                        message: ReliableServerMessage::PickedUpItem(entity_name, entity.to_bits(), slot.slot_name.clone()),
                    });
                },
                None => {},
            }

            

        },
        None => {

            rcon_spawn_entity(
                entity_name.to_string(),
                1,
                &mut commands,
                player_entity,
                player_handle,
                &mut rigid_body_positions,
                &mut net_console_commands,
                &gridmap_main,
                &mut used_names,
            );

        },
    }

    

}
