use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::{Commands, Query, Res, ResMut},
};
use bevy_transform::prelude::Transform;

use crate::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    console_commands::{
        events::{InputConsoleCommand, NetConsoleCommands},
        functions::rcon_spawn_held_entity::rcon_spawn_held_entity,
    },
    entity::resources::EntityDataResource,
    gridmap::resources::GridmapMain,
    inventory::components::Inventory,
    networking::resources::{ConsoleCommandVariantValues, ReliableServerMessage},
    pawn::{components::Pawn, resources::UsedNames},
};

pub fn inventory_item_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut commands: Commands,
    mut net_console_commands: EventWriter<NetConsoleCommands>,
    gridmap_main: Res<GridmapMain>,
    mut used_names: ResMut<UsedNames>,
    mut rigid_body_positions: Query<(&Transform, &Pawn)>,
    mut inventory_components: Query<&mut Inventory>,
    connected_players: Query<&ConnectedPlayer>,

    handle_to_entity: Res<HandleToEntity>,
    mut entity_data: ResMut<EntityDataResource>,
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
                    net_console_commands.send(NetConsoleCommands {
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
                ConsoleCommandVariantValues::String(value) => {
                    entity_name = value;
                }
                _ => {
                    return;
                }
            }

            let player_selector;

            match &console_command_event.command_arguments[1] {
                ConsoleCommandVariantValues::String(value) => {
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
            );
        }
    }
}
