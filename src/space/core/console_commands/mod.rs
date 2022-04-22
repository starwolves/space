use bevy_app::{App, Plugin};
use bevy_ecs::schedule::SystemSet;

use self::{
    events::NetConsoleCommands,
    resources::QueuedConsoleCommands,
    systems::{console_commands, console_commands_queue_clearer},
};

use super::networking::resources::ConsoleCommandVariant;

pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;
use bevy_app::CoreStage::PostUpdate;

pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QueuedConsoleCommands>()
            .add_event::<NetConsoleCommands>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new().with_system(console_commands_queue_clearer),
            )
            .add_system(console_commands);
    }
}

pub fn get_console_commands() -> Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)> {
    vec![
        (
            "rcon".to_string(),
            "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands".to_string(),
            vec![
                (   
                    "password".to_string(),
                    ConsoleCommandVariant::String
                ),
            ]
        ),
        (
            "rcon_status".to_string(),
            "For server administrators only. Check if the server has granted you the RCON status.".to_string(),
            vec![]
        ),
        (
            "spawn_entity".to_string(),
            "For server administrators only. Spawn in entities in proximity.".to_string(),
            vec![
                (
                    "entity_name".to_string(),
                    ConsoleCommandVariant::String
                ),
                (
                    "amount".to_string(),
                    ConsoleCommandVariant::Int
                ),
                (
                    "player_selector".to_string(),
                    ConsoleCommandVariant::String
                ),
            ]
        ),
        (
            "spawn_held_entity".to_string(),
            "For server administrators only. Spawn in held entities in hands or in proximity.".to_string(),
            vec![
                (
                    "entity_name".to_string(),
                    ConsoleCommandVariant::String
                ),
                (
                    "player_selector".to_string(),
                    ConsoleCommandVariant::String
                ),
            ]
        )
    ]
}
