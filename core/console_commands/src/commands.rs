use api::{
    console_commands::ConsoleCommandVariant,
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
};
use bevy::prelude::{info, ResMut};

pub struct NetConsoleCommands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetConsoleCommands {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub struct NetEntityConsole {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetEntityConsole {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Resource containing all registered custom console commands.
#[derive(Default)]
pub struct AllConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)>,
}
/// Initialize console commands.
pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "rcon".to_string(),
        "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands"
            .to_string(),
        vec![("password".to_string(), ConsoleCommandVariant::String)],
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
pub struct GiveAllRCON {
    pub give: bool,
}
