use crate::space_core::structs::network_messages::ReliableServerMessage;
pub struct NetLoadEntity {
    pub handle : u32,
    pub message : ReliableServerMessage
}
