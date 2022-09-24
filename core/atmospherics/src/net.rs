use api::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub(crate) struct NetAtmosphericsNotices {
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
pub(crate) struct NetMapHoverAtmospherics {
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
pub(crate) struct NetMapDisplayAtmospherics {
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
