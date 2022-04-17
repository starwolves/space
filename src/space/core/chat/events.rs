use bevy_ecs::entity::Entity;

use crate::space::core::networking::resources::ReliableServerMessage;

pub struct InputChatMessage {
    pub entity: Entity,
    pub message: String,
}

pub struct NetChatMessage {
    pub handle: u32,
    pub message: ReliableServerMessage,
}
