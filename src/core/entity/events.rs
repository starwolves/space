use crate::core::networking::resources::ReliableServerMessage;

use super::functions::raw_entity::RawEntity;

pub struct NetLoadEntity {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetShowcase {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetUnloadEntity {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetSendEntityUpdates {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct RawSpawnEvent {
    pub raw_entity: RawEntity,
}
