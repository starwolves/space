use bevy::prelude::EventWriter;

use crate::space_core::{events::net::net_unload_entity::NetUnloadEntity, structs::network_messages::ReliableServerMessage};

pub fn unload_entity(
    player_handle : u32,
    entity_id : u32,
    net_unload_entity : &mut EventWriter<NetUnloadEntity>,
) {

    net_unload_entity.send(
        NetUnloadEntity {
            handle: player_handle,
            message: ReliableServerMessage::UnloadEntity(
                entity_id,
                true
            )
        }
    );

}
