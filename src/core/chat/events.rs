use bevy_ecs::entity::Entity;

use crate::core::networking::resources::ReliableServerMessage;

pub struct InputChatMessage {
    pub entity: Entity,
    pub message: String,
}

pub struct NetChatMessage {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
