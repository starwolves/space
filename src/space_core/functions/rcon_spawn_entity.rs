use bevy::prelude::{Commands, Entity, EventWriter, Query, Res};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::{components::{pawn::Pawn}, events::net::net_console_commands::NetConsoleCommands, resources::gridmap_main::GridmapMain, structs::network_messages::ReliableServerMessage};

use super::{entity_spawn_position_for_player::entity_spawn_position_for_player, isometry_to_transform::isometry_to_transform, spawn_entity::spawn_entity};

pub fn rcon_spawn_entity(
    entity_name : String,
    mut spawn_amount : i64,
    commands : &mut Commands,
    player_entity : Entity,
    player_handle : u32,
    rigid_body_positions : &mut Query<(&RigidBodyPosition, &Pawn)>,
    net_console_commands : &mut EventWriter<NetConsoleCommands>,
    gridmap_main : &Res<GridmapMain>,
) {

    let player_position;
    let standard_character_component;

    match rigid_body_positions.get(player_entity) {
        Ok((position, pawn_component)) => {
            player_position = position.position.clone();
            standard_character_component = pawn_component;
        },
        Err(_rr) => {
            net_console_commands.send(NetConsoleCommands {
                handle: player_handle,
                message: ReliableServerMessage::ConsoleWriteLine(
                    "You need to board before you attempt to spawn in entities."
                    .to_string()
                ),
            });
            return;
        },
    }

    if spawn_amount > 5{
        spawn_amount = 5;
        net_console_commands.send(NetConsoleCommands {
            handle: player_handle,
            message: ReliableServerMessage::ConsoleWriteLine(
                "Capped amount to 5, maniac protection."
                .to_string()
            ),
        });
    }


    let spawn_position = 
    entity_spawn_position_for_player(
    isometry_to_transform(
    player_position,
        ),
        &standard_character_component.facing_direction,
        gridmap_main,
    );
    
    for _i in 0..spawn_amount {

        spawn_entity(
            entity_name.clone(),
            spawn_position,
            commands,
            true,
        );

    }

    

}
