use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate, Startup, Update};
use networking::messaging::{register_reliable_message, MessageSender};
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

#[derive(Default)]
pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(PreUpdate, incoming_messages)
                .add_event::<InputConsoleCommand>()
                .add_systems(
                    Update,
                    configure
                        .in_set(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                );
        }
        app.add_systems(
            Startup,
            (
                initialize_console_commands
                    .in_set(ConsoleCommandsLabels::Finalize)
                    .in_set(StartupLabels::ConsoleCommands),
                initialize_console_commands_2
                    .before(ConsoleCommandsLabels::Finalize)
                    .in_set(BuildingLabels::TriggerBuild),
            ),
        )
        .init_resource::<AllConsoleCommands>();
        register_reliable_message::<ConsoleCommandsClientMessage>(app, MessageSender::Client);
        register_reliable_message::<ConsoleCommandsServerMessage>(app, MessageSender::Server);
    }
}
