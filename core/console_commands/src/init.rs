use bevy::prelude::info;
use bevy::prelude::ResMut;
use networking::server::GodotVariant;

use crate::commands::AllConsoleCommands;

/// Initialize console commands.
#[cfg(feature = "server")]
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

    info!("Loaded {} different console commands.", commands.list.len());
}
