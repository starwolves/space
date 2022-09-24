use api::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub(crate) struct NetDropCurrentItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetDropCurrentItem {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub(crate) struct NetPickupWorldItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetPickupWorldItem {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub(crate) struct NetSwitchHands {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetSwitchHands {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub(crate) struct NetTakeOffItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetTakeOffItem {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub(crate) struct NetWearItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetWearItem {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
pub(crate) struct NetThrowItem {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetThrowItem {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
