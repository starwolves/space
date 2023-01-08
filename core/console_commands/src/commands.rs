use bevy::prelude::Resource;
use bevy::prelude::{Entity, SystemLabel};
use networking::server::GodotVariant;
use networking::server::GodotVariantValues;

/// Resource containing all registered custom console commands.
#[derive(Default, Resource)]

pub struct AllConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, GodotVariant)>)>,
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum ConsoleCommandsLabels {
    Finalize,
}

/// Client input console command message event.

pub struct InputConsoleCommand {
    /// The connection handle tied to the entity performing the command.
    pub handle_option: Option<u64>,
    /// The entity performing the command.
    pub entity: Entity,
    /// The command name.
    pub command_name: String,
    /// The passed arguments to the command as variants.
    pub command_arguments: Vec<GodotVariantValues>,
}
