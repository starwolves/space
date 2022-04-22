use bevy_app::{App, Plugin};

use self::{
    events::{InputConsoleCommand, NetConsoleCommands},
    systems::console_commands,
};

pub mod events;
pub mod functions;
pub mod systems;

pub struct ConsoleCommandsPlugin;

impl Plugin for ConsoleCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputConsoleCommand>()
            .add_event::<NetConsoleCommands>()
            .add_system(console_commands);
    }
}
