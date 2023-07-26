use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};
use networking::messaging::{register_reliable_message, MessageSender};
use player::plugin::ConfigurationLabel;
use resources::{
    is_server::is_server,
    sets::{BuildingSet, MainSet, StartupSet},
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
        if is_server() {
            app.add_systems(FixedUpdate, incoming_messages.in_set(MainSet::PreUpdate))
                .add_event::<InputConsoleCommand>()
                .add_systems(
                    FixedUpdate,
                    configure
                        .in_set(ConfigurationLabel::Main)
                        .in_set(MainSet::Update)
                        .after(ConfigurationLabel::SpawnEntity),
                );
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
        register_reliable_message::<ConsoleCommandsClientMessage>(app, MessageSender::Client);
        register_reliable_message::<ConsoleCommandsServerMessage>(app, MessageSender::Server);
    }
}
