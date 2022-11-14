use bevy::prelude::SystemLabel;
use bevy::prelude::{info, ResMut};
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::{GodotVariant, ReliableServerMessage};
use networking_macros::NetMessage;

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetConsoleCommands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetEntityConsole {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

/// Resource containing all registered custom console commands.
#[derive(Default)]
#[cfg(feature = "server")]
pub struct AllConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, GodotVariant)>)>,
}
/// Initialize console commands.
#[cfg(feature = "server")]
pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "rcon".to_string(),
        "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands"
            .to_string(),
        vec![("password".to_string(), GodotVariant::String)],
    ));

    commands.list.push((
        "rconStatus".to_string(),
        "For server administrators only. Check if the server has granted you the RCON status."
            .to_string(),
        vec![],
    ));

    info!("Loaded {} different console commands.", commands.list.len());
}
/// Resource with the configuration whether new players should be given RCON upon connection.
#[derive(Default)]
#[cfg(feature = "server")]
pub struct GiveAllRCON {
    pub give: bool,
}
/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "server")]
pub enum ConsoleCommandsLabels {
    Finalize,
}

#[cfg(feature = "server")]
pub const CONSOLE_SUCCESS_COLOR: &str = "#3cff00";
#[cfg(feature = "server")]
pub const CONSOLE_ERROR_COLOR: &str = "#ff6600";
