use networking::messages::PendingMessage;
use networking::messages::PendingNetworkMessage;
use networking::messages::ReliableServerMessage;
use networking_macros::NetMessage;
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetGridmapUpdates {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetProjectileFOV {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
