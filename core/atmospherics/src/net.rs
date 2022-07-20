use shared::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub struct NetAtmosphericsNotices {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetAtmosphericsNotices {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub struct NetMapHoverAtmospherics {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetMapHoverAtmospherics {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub struct NetMapDisplayAtmospherics {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetMapDisplayAtmospherics {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
