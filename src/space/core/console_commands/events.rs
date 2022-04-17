use bevy_ecs::entity::Entity;

use crate::space::core::networking::resources::{
    ConsoleCommandVariantValues, ReliableServerMessage,
};

pub struct NetConsoleCommands {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct InputConsoleCommand {
    pub handle: u32,
    pub entity: Entity,
    pub command_name: String,
    pub command_arguments: Vec<ConsoleCommandVariantValues>,
}
