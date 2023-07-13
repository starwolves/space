use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup, Update};
use hud::communication::console::console_input;
use resources::is_server::is_server;

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
            app.add_systems(Update, (rcon_console_commands, export_map, coords))
                .insert_resource::<GiveAllRCON>(GiveAllRCON {
                    give: self.give_all_rcon,
                });
        } else {
            app.add_systems(Startup, add_help_command)
                .add_systems(Update, help_command.after(console_input));
        }
        app.add_systems(Startup, add_export_map_command);
    }
}
