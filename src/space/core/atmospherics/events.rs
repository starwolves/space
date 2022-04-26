use crate::space::core::networking::resources::{NetMessageType, ReliableServerMessage};

pub struct NetMapDisplayAtmospherics {
    pub handle: u32,
    pub message: NetMessageType,
}

pub struct NetMapHoverAtmospherics {
    pub handle: u32,
    pub message: NetMessageType,
}

pub struct NetAtmosphericsNotices {
    pub handle: u32,
    pub message: ReliableServerMessage,
}
