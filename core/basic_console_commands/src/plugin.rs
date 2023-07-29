use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};
use console_commands::commands::ConsoleCommandsSet;
use hud::communication::{console::console_input, input::ConsoleCommandsClientSet};
use resources::{is_server::is_server, sets::MainSet};

use crate::{
    commands::{coords, rcon_console_commands, GiveAllRCON},
    gridmap::{add_export_map_command, export_map},
    help::{add_help_command, help_command},
};

#[derive(Default)]
pub struct BasicConsoleCommandsPlugin {
    pub give_all_rcon: bool,
}

impl Plugin for BasicConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                (rcon_console_commands, export_map, coords)
                    .in_set(MainSet::Update)
                    .after(ConsoleCommandsSet::Input),
            )
            .insert_resource::<GiveAllRCON>(GiveAllRCON {
                give: self.give_all_rcon,
            });
        } else {
            app.add_systems(Startup, add_help_command).add_systems(
                FixedUpdate,
                help_command
                    .after(ConsoleCommandsClientSet::Submit)
                    .after(console_input)
                    .before(ConsoleCommandsClientSet::Display)
                    .in_set(MainSet::Update),
            );
        }
        app.add_systems(Startup, add_export_map_command);
    }
}
