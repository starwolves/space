use bevy_ecs::{
    entity::Entity,
    prelude::{FromWorld, World},
};

use crate::space::core::networking::resources::ConsoleCommandVariantValues;

pub struct InputConsoleCommand {
    pub handle: u32,
    pub entity: Entity,
    pub command_name: String,
    pub command_arguments: Vec<ConsoleCommandVariantValues>,
}

pub struct QueuedConsoleCommands {
    pub queue: Vec<InputConsoleCommand>,
}

impl FromWorld for QueuedConsoleCommands {
    fn from_world(_world: &mut World) -> Self {
        QueuedConsoleCommands { queue: vec![] }
    }
}
