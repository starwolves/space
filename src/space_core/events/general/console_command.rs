use bevy::prelude::Entity;

use crate::space_core::resources::network_messages::ConsoleCommandVariantValues;

pub struct InputConsoleCommand {
    pub handle : u32,
    pub entity : Entity,
    pub command_name : String,
    pub command_arguments : Vec<ConsoleCommandVariantValues>,
}
