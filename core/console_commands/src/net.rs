use networking::server::{GodotVariant, GodotVariantValues};
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ConsoleCommandsServerMessage {
    ConsoleWriteLine(String),
    ConfigConsoleCommands(Vec<(String, String, Vec<(String, GodotVariant)>)>),
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum ConsoleCommandsClientMessage {
    ConsoleCommand(String, Vec<GodotVariantValues>),
}