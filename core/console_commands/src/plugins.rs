use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet};
use networking::messages::net_system;
use shared::{
    console_commands::ConsoleCommandsLabels,
    data::{PostUpdateLabels, StartupLabels},
};

use crate::{
    commands::{initialize_console_commands, AllConsoleCommands, NetEntityConsole},
    rcon::GiveAllRCON,
};
use bevy::app::CoreStage::PostUpdate;

use super::commands::NetConsoleCommands;

#[derive(Default)]
pub struct ConsoleCommandsPlugin {
    pub give_all_rcon: bool,
}

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AllConsoleCommands>()
            .add_event::<NetConsoleCommands>()
            .add_event::<NetEntityConsole>()
            .add_startup_system(
                initialize_console_commands
                    .label(ConsoleCommandsLabels::Finalize)
                    .label(StartupLabels::ConsoleCommands),
            )
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetConsoleCommands>)
                    .with_system(net_system::<NetEntityConsole>),
            )
            .insert_resource::<GiveAllRCON>(GiveAllRCON {
                give: self.give_all_rcon,
            });
    }
}
