use bevy::prelude::{info, Res, ResMut};
use console_commands::commands::AllConsoleCommands;
use networking::messages::GodotVariant;

use crate::meta::EntityDataResource;

/// Print startup entity data to console.
#[cfg(feature = "server")]
pub(crate) fn startup_entities(entity_data: Res<EntityDataResource>) {
    info!("Loaded {} different entity types.", entity_data.data.len());
}

/// Initialize console commands.
#[cfg(feature = "server")]
pub(crate) fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
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
