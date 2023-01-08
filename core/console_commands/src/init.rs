use bevy::prelude::info;
use bevy::prelude::ResMut;
use networking::server::GodotVariant;

use crate::commands::AllConsoleCommands;

/// Initialize console commands.

pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "rcon".to_string(),
        "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands"
            .to_string(),
        vec![("password".to_string(), GodotVariant::String)],
    ));

    commands.list.push((
        "rconStatus".to_string(),
        "For server administrators only. Check if the server has granted you the RCON status."
            .to_string(),
        vec![],
    ));

    info!("Loaded {} console commands.", commands.list.len());
}

/// Initialize console commands.

pub(crate) fn initialize_console_commands_2(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "spawn".to_string(),
        "For server administrators only. Spawn in entities in proximity.".to_string(),
        vec![
            ("entity_name".to_string(), GodotVariant::String),
            ("amount".to_string(), GodotVariant::Int),
            ("player_selector".to_string(), GodotVariant::String),
        ],
    ));
}
