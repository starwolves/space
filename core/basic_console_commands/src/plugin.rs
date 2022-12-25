use bevy::prelude::{App, Plugin};
use resources::is_server::is_server;

use crate::commands::{rcon_console_commands, GiveAllRCON};

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
        }
    }
}
