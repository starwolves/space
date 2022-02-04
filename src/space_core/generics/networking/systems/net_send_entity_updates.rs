use crate::space_core::generics::networking::resources::ReliableServerMessage;


pub struct NetSendEntityUpdates {
    pub handle : u32,
    pub message : ReliableServerMessage
}
