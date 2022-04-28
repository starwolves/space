use bevy_app::EventWriter;
use bevy_ecs::entity::Entity;

use crate::core::{entity::events::NetUnloadEntity, networking::resources::ReliableServerMessage};

pub fn unload_entity(
    player_handle: u32,
    entity_id: Entity,
    net_unload_entity: &mut EventWriter<NetUnloadEntity>,
    unload_entirely: bool,
) {
    net_unload_entity.send(NetUnloadEntity {
        handle: player_handle,
        message: ReliableServerMessage::UnloadEntity(entity_id.to_bits(), unload_entirely),
    });
}
