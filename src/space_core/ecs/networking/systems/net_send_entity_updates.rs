use crate::space_core::ecs::networking::resources::ReliableServerMessage;


pub struct NetSendEntityUpdates {
    pub handle : u32,
    pub message : ReliableServerMessage
}
