use api::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub struct NetDropCurrentItem {
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
pub struct NetPickupWorldItem {
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
pub struct NetSwitchHands {
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
pub struct NetTakeOffItem {
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
pub struct NetWearItem {
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
pub struct NetThrowItem {
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
