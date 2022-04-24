use bevy_app::{App, Plugin};
use bevy_ecs::{
    schedule::{ParallelSystemDescriptorCoercion, SystemLabel, SystemSet},
    system::ResMut,
};
use bevy_log::info;

use crate::space::StartupLabels;

use self::{
    events::NetConsoleCommands,
    resources::{ConsoleCommands, QueuedConsoleCommands},
    systems::{console_commands, console_commands_queue_clearer},
};

use super::networking::resources::ConsoleCommandVariant;

pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;
use bevy_app::CoreStage::PostUpdate;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ConsoleCommandsLabels {
    Finalize,
}

pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QueuedConsoleCommands>()
            .init_resource::<ConsoleCommands>()
            .add_event::<NetConsoleCommands>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new().with_system(console_commands_queue_clearer),
            )
            .add_system(console_commands)
            .add_startup_system(
                initialize_console_commands
                    .label(ConsoleCommandsLabels::Finalize)
                    .label(StartupLabels::ConsoleCommands),
            );
    }
}

pub fn initialize_console_commands(mut commands: ResMut<ConsoleCommands>) {
    commands.list.push((
        "rcon".to_string(),
        "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands"
            .to_string(),
        vec![("password".to_string(), ConsoleCommandVariant::String)],
    ));

    commands.list.push((
        "rcon_status".to_string(),
        "For server administrators only. Check if the server has granted you the RCON status."
            .to_string(),
        vec![],
    ));

    info!("Loaded {} different console commands.", commands.list.len());
}
