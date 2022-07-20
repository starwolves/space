use api::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub struct NetConstructionTool {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetConstructionTool {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
