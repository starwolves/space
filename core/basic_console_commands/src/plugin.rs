use bevy::prelude::{App, IntoSystemConfig, Plugin};
use hud::communication::console::console_input;
use resources::is_server::is_server;

use crate::{
    commands::{rcon_console_commands, GiveAllRCON},
    help::{add_help_command, help_command},
};

#[derive(Default)]
pub struct BasicConsoleCommandsPlugin {
    pub give_all_rcon: bool,
}

impl Plugin for BasicConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(rcon_console_commands)
                .insert_resource::<GiveAllRCON>(GiveAllRCON {
                    give: self.give_all_rcon,
                });
        } else {
            app.add_startup_system(add_help_command)
                .add_system(help_command.after(console_input));
        }
    }
}
