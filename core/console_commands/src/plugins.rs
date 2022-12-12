use std::env;

use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use networking::messaging::{init_reliable_message, MessageSender};
use player::plugin::ConfigurationLabel;
use resources::labels::{StartupLabels, SummoningLabels};

use crate::{
    commands::{AllConsoleCommands, ConsoleCommandsLabels, InputConsoleCommand},
    connections::configure,
    init::{initialize_console_commands, initialize_console_commands_2},
    networking::{incoming_messages, ConsoleCommandsClientMessage, ConsoleCommandsServerMessage},
};

use bevy::app::CoreStage::PreUpdate;
#[derive(Default)]
pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
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
                        .label(SummoningLabels::TriggerSummon),
                );
        }

        init_reliable_message::<ConsoleCommandsClientMessage>(app, MessageSender::Client);
        init_reliable_message::<ConsoleCommandsServerMessage>(app, MessageSender::Server);
    }
}
