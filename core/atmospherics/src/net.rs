use networking::messages::PendingMessage;
use networking::messages::PendingNetworkMessage;
use networking::messages::ReliableServerMessage;
use networking_macros::NetMessage;
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetAtmosphericsNotices {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetMapHoverAtmospherics {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetMapDisplayAtmospherics {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
