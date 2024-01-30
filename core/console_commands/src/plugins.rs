use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup};
use networking::messaging::{register_reliable_message, MessageSender};
use player::{connections::process_response, plugin::ConfigurationLabel};
use resources::{
    modes::is_server_mode,
    ordering::{BuildingSet, StartupSet, Update},
};

use crate::{
    commands::{AllConsoleCommands, ConsoleCommandsSet, InputConsoleCommand},
    connections::configure,
    init::{initialize_console_commands, initialize_console_commands_2},
    net::{ConsoleCommandsClientMessage, ConsoleCommandsServerMessage},
    networking::incoming_messages,
};

#[derive(Default)]
pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (
                    incoming_messages.in_set(ConsoleCommandsSet::Input),
                    configure
                        .after(process_response)
                        .in_set(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                ),
            )
            .add_event::<InputConsoleCommand>();
        }
        app.add_systems(
            Startup,
            (
                initialize_console_commands
                    .in_set(ConsoleCommandsSet::Finalize)
                    .in_set(StartupSet::ConsoleCommands),
                initialize_console_commands_2
                    .before(ConsoleCommandsSet::Finalize)
                    .in_set(BuildingSet::TriggerBuild),
            ),
        )
        .init_resource::<AllConsoleCommands>();
        register_reliable_message::<ConsoleCommandsClientMessage>(app, MessageSender::Client, true);
        register_reliable_message::<ConsoleCommandsServerMessage>(app, MessageSender::Server, true);
    }
}
