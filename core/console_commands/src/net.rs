use serde::{Deserialize, Serialize};
use typename::TypeName;
use ui::text::NetTextSection;

use crate::commands::ConsoleCommand;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ConsoleCommandsServerMessage {
    ConsoleWriteLine(ConsoleLine),
    ConfigConsoleCommands(Vec<ConsoleCommand>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConsoleLine {
    pub sections: Vec<NetTextSection>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ConsoleCommandsClientMessage {
    ConsoleCommand(ClientSideConsoleInput),
}

/// Event for new input console commands.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientSideConsoleInput {
    pub command: String,
    pub args: Vec<String>,
}

impl ClientSideConsoleInput {
    pub fn from_string(str: String) -> Self {
        let mut split: Vec<&str> = str.split(" ").collect();

        let base = split.first().unwrap().clone();

        split.remove(0);

        let mut args = vec![];
        for s in split.iter() {
            args.push(s.to_string());
        }

        ClientSideConsoleInput {
            command: base.to_string(),
            args,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{} {}", self.command, self.args.join(" "))
    }
}
