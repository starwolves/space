use bevy::prelude::{Entity, SystemSet};
use bevy::prelude::{Event, Resource};
use networking::server::ConsoleArgVariant;
use serde::{Deserialize, Serialize};

use crate::net::ClientSideConsoleInput;

/// Resource containing all registered custom console commands.
#[derive(Default, Resource)]
pub struct AllConsoleCommands {
    pub list: Vec<ConsoleCommand>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsoleCommand {
    pub base: String,
    pub description: String,
    pub args: Vec<(String, ConsoleArgVariant)>,
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum ConsoleCommandsSet {
    Finalize,
}

/// Client input console command message event.
#[derive(Event)]
pub struct InputConsoleCommand {
    /// The connection handle tied to the entity performing the command.
    pub handle_option: Option<u64>,
    /// The entity performing the command.
    pub entity: Entity,
    pub input: ClientSideConsoleInput,
}
