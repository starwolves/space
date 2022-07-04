#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ConsoleCommandsLabels {
    Finalize,
}

use bevy::prelude::{info, App, ParallelSystemDescriptorCoercion, Plugin, ResMut, SystemLabel};

use crate::core::{
    networking::networking::ConsoleCommandVariant,
    space_plugin::plugin::{PostUpdateLabels, StartupLabels},
};

use super::commands::{
    console_commands, net_system, AllConsoleCommands, InputConsoleCommand, NetConsoleCommands,
};

pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllConsoleCommands>()
            .add_event::<NetConsoleCommands>()
            .add_event::<InputConsoleCommand>()
            .add_system(console_commands)
            .add_startup_system(
                initialize_console_commands
                    .label(ConsoleCommandsLabels::Finalize)
                    .label(StartupLabels::ConsoleCommands),
            )
            .add_system_to_stage(
                PostUpdate,
                net_system
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net),
            );
    }
}
use bevy::app::CoreStage::PostUpdate;
pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
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
