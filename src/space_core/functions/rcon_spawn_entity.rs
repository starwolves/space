use bevy::prelude::{Commands, Entity, EventWriter, Query};
use bevy_rapier3d::prelude::RigidBodyPosition;

use crate::space_core::{events::net::net_console_commands::NetConsoleCommands, structs::network_messages::ReliableServerMessage};

use super::spawn_entity::spawn_entity;

pub fn rcon_spawn_entity(
    entity_name : String,
    spawn_amount : i64,
    commands : &mut Commands,
    player_entity : Entity,
    player_handle : u32,
    rigid_body_positions : &mut Query<&RigidBodyPosition>,
    net_console_commands : &mut EventWriter<NetConsoleCommands>,
) {


    let player_position;

    match rigid_body_positions.get(player_entity) {
        Ok(position) => {
            player_position = position.position;
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

    // Obtain entity's default position.

    /*player_position
    
    for i in 0..spawn_amount {

        spawn_entity(
            entity_name,
            ,
            &mut commands,
        );

    }*/

    

}
