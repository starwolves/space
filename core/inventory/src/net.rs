use api::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};
use networking_macros::NetMessage;
#[derive(NetMessage)]
pub(crate) struct NetDropCurrentItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetPickupWorldItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetSwitchHands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetTakeOffItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetWearItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
pub(crate) struct NetThrowItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
