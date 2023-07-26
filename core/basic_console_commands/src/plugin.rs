use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};
use hud::communication::console::console_input;
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
                (rcon_console_commands, export_map, coords).in_set(MainSet::Update),
            )
            .insert_resource::<GiveAllRCON>(GiveAllRCON {
                give: self.give_all_rcon,
            });
        } else {
            app.add_systems(Startup, add_help_command).add_systems(
                FixedUpdate,
                help_command.after(console_input).in_set(MainSet::Update),
            );
        }
        app.add_systems(Startup, add_export_map_command);
    }
}
