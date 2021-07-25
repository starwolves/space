use bevy::prelude::{Entity, EventWriter};

use crate::space_core::{events::net::net_unload_entity::NetUnloadEntity, resources::network_messages::ReliableServerMessage};

pub fn unload_entity(
    player_handle : u32,
    entity_id : Entity,
    net_unload_entity : &mut EventWriter<NetUnloadEntity>,
    unload_entirely : bool,
) {

    net_unload_entity.send(
        NetUnloadEntity {
            handle: player_handle,
            message: ReliableServerMessage::UnloadEntity(
                entity_id.to_bits(),
                unload_entirely
            )
        }
    );

}
