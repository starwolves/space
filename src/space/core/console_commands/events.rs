use crate::space::core::networking::resources::ReliableServerMessage;

pub struct NetConsoleCommands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
