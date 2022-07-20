use shared::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub struct NetCounterWindow {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetCounterWindow {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
