use bevy::prelude::{warn, Entity, Res, ResMut};

use player::names::UsedNames;

use bevy::prelude::EventWriter;
use console_commands::net::{ConsoleCommandsServerMessage, ConsoleLine};
use networking::server::OutgoingReliableServerMessage;
use ui::{
    fonts::{Fonts, SOURCECODE_REGULAR_FONT},
    text::{NetTextSection, COMMUNICATION_FONT_SIZE, CONSOLE_ERROR_COLOR},
};

/// Player selector to entities.

pub(crate) fn player_selector_to_entities(
    command_executor_entity: Entity,
    command_executor_handle_option: Option<u64>,
    mut player_selector: &str,
    used_names: &mut ResMut<UsedNames>,
    server: &mut EventWriter<OutgoingReliableServerMessage<ConsoleCommandsServerMessage>>,
    fonts: &Res<Fonts>,
) -> Vec<Entity> {
    if player_selector == "*" {
        return used_names.names.values().copied().collect();
    } else if player_selector == "@me" {
        return vec![command_executor_entity];
    }

    let precise_match = if (player_selector.starts_with('"') && player_selector.ends_with('"'))
        || (player_selector.starts_with('\'') && player_selector.ends_with('\''))
    {
        // Remove surrounding quotes
        let mut chars = player_selector.chars();
        chars.next();
        chars.next_back();
        player_selector = chars.as_str();
        true
    } else {
        false
    };

    let matching_names: Vec<_> = used_names
        .names
        .iter()
        .filter(|(player_name, _)| {
            let player_name_lower = player_name.to_lowercase();
            let player_selector = player_selector.to_lowercase();
            let val;
            if precise_match {
                val = player_name_lower == player_selector;
            } else {
                val = player_name_lower.contains(&player_selector);
            }

            val
        })
        .collect();

    let message = match &matching_names[..] {
        [(_, &entity)] => return vec![entity],
        [] => {
            format!("Couldn't find player \"{player_selector}\"")
        }
        [conflicts @ ..] => {
            let mut names = String::new();
            for (name, _entity) in conflicts.iter() {
                names.push_str(name);
                names.push('\n');
            }

            format!("Player selector \"{player_selector}\" is not specific enough.\n{names}")
        }
    };
    match command_executor_handle_option {
        Some(handle) => {
            let section = NetTextSection {
                text: message.to_string(),
                font: *fonts.inv_map.get(SOURCECODE_REGULAR_FONT).unwrap(),
                font_size: COMMUNICATION_FONT_SIZE,
                color: CONSOLE_ERROR_COLOR,
            };

            server.send(OutgoingReliableServerMessage {
                handle,
                message: ConsoleCommandsServerMessage::ConsoleWriteLine(ConsoleLine {
                    sections: vec![section],
                }),
            });
        }
        None => {
            warn!("Couldnt find connection handle.");
        }
    }
    return vec![];
}
