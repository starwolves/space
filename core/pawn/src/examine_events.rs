use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetPawn {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
