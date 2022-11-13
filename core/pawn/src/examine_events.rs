use networking::messages::PendingMessage;
use networking::messages::PendingNetworkMessage;
use networking::messages::ReliableServerMessage;
use networking_macros::NetMessage;
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetPawnExamine {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
