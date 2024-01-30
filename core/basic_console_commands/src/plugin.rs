use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup};
use console_commands::commands::ConsoleCommandsSet;
use hud::communication::{console::console_input, input::ConsoleCommandsClientSet};
use resources::{modes::is_server_mode, ordering::Update};

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
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (rcon_console_commands, export_map, coords).after(ConsoleCommandsSet::Input),
            )
            .insert_resource::<GiveAllRCON>(GiveAllRCON {
                give: self.give_all_rcon,
            });
        } else {
            app.add_systems(Startup, add_help_command).add_systems(
                Update,
                help_command
                    .after(ConsoleCommandsClientSet::Submit)
                    .after(console_input)
                    .before(ConsoleCommandsClientSet::Display),
            );
        }
        app.add_systems(Startup, add_export_map_command);
    }
}
