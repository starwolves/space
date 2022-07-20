use shared::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub struct NetGridmapUpdates {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetGridmapUpdates {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub struct NetProjectileFOV {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetProjectileFOV {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
