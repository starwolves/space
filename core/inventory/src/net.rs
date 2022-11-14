use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetDropCurrentItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetPickupWorldItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetSwitchHands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetTakeOffItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetWearItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetThrowItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
