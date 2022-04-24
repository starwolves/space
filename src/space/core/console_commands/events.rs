use crate::space::core::networking::resources::ReliableServerMessage;

pub struct NetConsoleCommands {
    pub handle: u32,
    pub message: ReliableServerMessage,
}
