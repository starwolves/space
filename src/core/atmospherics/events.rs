use crate::core::networking::resources::ReliableServerMessage;

pub struct NetMapDisplayAtmospherics {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetMapHoverAtmospherics {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetAtmosphericsNotices {
    pub handle: u32,
    pub message: ReliableServerMessage,
}
