use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use networking::messaging::{init_reliable_message, MessageSender};
use player::plugin::ConfigurationLabel;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, StartupLabels},
};

use crate::{
    commands::{AllConsoleCommands, ConsoleCommandsLabels, InputConsoleCommand},
    connections::configure,
    init::{initialize_console_commands, initialize_console_commands_2},
    net::{ConsoleCommandsClientMessage, ConsoleCommandsServerMessage},
    networking::incoming_messages,
};

use bevy::app::CoreStage::PreUpdate;
#[derive(Default)]
pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.init_resource::<AllConsoleCommands>()
                .add_startup_system(
                    initialize_console_commands
                        .label(ConsoleCommandsLabels::Finalize)
                        .label(StartupLabels::ConsoleCommands),
                )
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<InputConsoleCommand>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                )
                .add_startup_system(
                    initialize_console_commands_2
                        .before(ConsoleCommandsLabels::Finalize)
                        .label(BuildingLabels::TriggerBuild),
                );
        }

        init_reliable_message::<ConsoleCommandsClientMessage>(app, MessageSender::Client);
        init_reliable_message::<ConsoleCommandsServerMessage>(app, MessageSender::Server);
    }
}
