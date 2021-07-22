use bevy::prelude::Entity;

use crate::space_core::structs::network_messages::ConsoleCommandVariantValues;

pub struct ConsoleCommand {
    pub handle : u32,
    pub entity : Entity,
    pub command_name : String,
    pub command_arguments : Vec<ConsoleCommandVariantValues>,
}
