use crate::space::core::networking::resources::ReliableServerMessage;

pub struct NetLoadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetShowcase {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetUnloadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetSendEntityUpdates {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
