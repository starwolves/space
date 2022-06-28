use bevy_ecs::entity::Entity;

use crate::core::networking::resources::{ConsoleCommandVariantValues, ReliableServerMessage};

pub struct NetConsoleCommands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct InputConsoleCommand {
    pub handle_option: Option<u64>,
    pub entity: Entity,
    pub command_name: String,
    pub command_arguments: Vec<ConsoleCommandVariantValues>,
}
