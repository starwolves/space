use api::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};
use networking_macros::NetMessage;
#[derive(NetMessage)]
pub(crate) struct NetConstructionTool {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
