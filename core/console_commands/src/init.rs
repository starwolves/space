use bevy::log::info;
use bevy::prelude::ResMut;
use networking::server::ConsoleArgVariant;

use crate::commands::AllConsoleCommands;
use crate::commands::ConsoleCommand;

/// Initialize console commands.

pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push(ConsoleCommand {
        base: "rcon".to_string(),
        description: "Obtaining rcon status allows for usage of rcon commands".to_string(),
        args: vec![("password".to_string(), ConsoleArgVariant::String)],
    });

    commands.list.push(ConsoleCommand {
        base: "rconStatus".to_string(),
        description: "Check if the server has granted you the RCON status.".to_string(),
        args: vec![],
    });

    info!("Loaded {} console commands.", commands.list.len());
}

/// Initialize console commands.

pub(crate) fn initialize_console_commands_2(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push(ConsoleCommand {
        base: "spawn".to_string(),
        description: "Spawn in entities in proximity.".to_string(),
        args: vec![
            ("entity_name".to_string(), ConsoleArgVariant::String),
            ("amount".to_string(), ConsoleArgVariant::Int),
            ("player_selector".to_string(), ConsoleArgVariant::String),
        ],
    });

    commands.list.push(ConsoleCommand {
        base: "coords".to_string(),
        description: "Get your current 3D world coordinates.".to_string(),
        args: vec![],
    });
}
