use crate::space_core::structs::network_messages::ReliableServerMessage;
pub struct NetSendWorldEnvironment {
    pub handle : u32,
    pub message : ReliableServerMessage
}
