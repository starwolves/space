use bevy_ecs::prelude::{FromWorld, World};

use crate::core::networking::resources::ConsoleCommandVariant;

pub struct AllConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)>,
}

impl FromWorld for AllConsoleCommands {
    fn from_world(_world: &mut World) -> Self {
        AllConsoleCommands { list: vec![] }
    }
}
