use bevy_ecs::{
    entity::Entity,
    prelude::{FromWorld, World},
};

use crate::space::core::networking::resources::{
    ConsoleCommandVariant, ConsoleCommandVariantValues,
};

pub struct InputConsoleCommand {
    pub handle_option: Option<u64>,
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

pub struct ConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)>,
}

impl FromWorld for ConsoleCommands {
    fn from_world(_world: &mut World) -> Self {
        ConsoleCommands { list: vec![] }
    }
}
