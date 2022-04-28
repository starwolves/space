use bevy_ecs::{
    entity::Entity,
    prelude::{FromWorld, World},
};

use crate::core::networking::resources::{ConsoleCommandVariant, ConsoleCommandVariantValues};

pub struct InputConsoleCommand {
    pub handle_option: Option<u32>,
    pub entity: Entity,
    pub command_name: String,
    pub command_arguments: Vec<ConsoleCommandVariantValues>,
}

pub struct ConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)>,
}

impl FromWorld for ConsoleCommands {
    fn from_world(_world: &mut World) -> Self {
        ConsoleCommands { list: vec![] }
    }
}
