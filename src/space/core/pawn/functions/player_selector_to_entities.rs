use bevy::prelude::{Entity, EventWriter, ResMut};

use crate::space::core::{
    networking::resources::ReliableServerMessage,
    pawn::{events::NetConsoleCommands, resources::UsedNames},
};

pub fn player_selector_to_entities(
    command_executor_entity: Entity,
    command_executor_handle: u32,
    player_selector_input: &str,
    used_names: &mut ResMut<UsedNames>,
    net_console_commands: &mut EventWriter<NetConsoleCommands>,
) -> Vec<Entity> {
    let mut player_selector = player_selector_input.clone().to_string();

    let mut target_entities = vec![];

    if player_selector == "*" {
        for entity in used_names.names.values() {
            target_entities.push(*entity);
        }
    } else if player_selector == "@me" {
        target_entities.push(command_executor_entity);
    } else {
        // Assume we only target one player.

        let mut found_one_match = false;
        let mut conflicting_names = vec![];

        let precise_match;

        if (player_selector.starts_with("\"") || player_selector.starts_with("\'"))
            && (player_selector.ends_with("\"") || player_selector.ends_with("\'"))
        {
            precise_match = true;
            player_selector = player_selector.replace("\"", "");
        } else {
            precise_match = false;
        }

        for (player_name, entity) in used_names.names.iter() {
            let matcher;

            if precise_match == false {
                matcher = player_name
                    .to_lowercase()
                    .contains(&player_selector.to_lowercase());
            } else {
                matcher = player_name.to_lowercase() == player_selector.to_lowercase();
            }

            if matcher {
                if found_one_match {
                    found_one_match = false;
                    conflicting_names.push((player_name, entity));
                } else {
                    found_one_match = true;
                    conflicting_names.push((player_name, entity));
                }
            }
        }

        if found_one_match {
            target_entities.push(*conflicting_names[0].1);
        } else {
            let mut conflicting_message = "[color=#ff6600]Player selector \"".to_string()
                + &player_selector
                + " \" is not specific enough:\n";

            for (name, _entity) in conflicting_names.iter() {
                conflicting_message = conflicting_message + name + "\n";
            }

            conflicting_message = conflicting_message + "[/color]";

            net_console_commands.send(NetConsoleCommands {
                handle: command_executor_handle,
                message: ReliableServerMessage::ConsoleWriteLine(conflicting_message),
            });
        }
    }

    target_entities
}
