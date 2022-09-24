use api::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub(crate) struct NetGridmapUpdates {
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
pub(crate) struct NetProjectileFOV {
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
