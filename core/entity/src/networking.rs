use bevy::prelude::ResMut;

use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::server::ReliableClientMessage;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;

use crate::examine::InputExamineEntity;
use bevy::prelude::Entity;
use bevy::prelude::EventWriter;
use bevy::prelude::Res;
use networking::server::HandleToEntity;

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_examine_entity: EventWriter<InputExamineEntity>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<ReliableClientMessage, _> =
                bincode::deserialize(&message);
            let client_message;
            match client_message_result {
                Ok(x) => {
                    client_message = x;
                }
                Err(_rr) => {
                    warn!("Received invalid client message.");
                    continue;
                }
            }

            match client_message {
                ReliableClientMessage::ExamineEntity(entity_id) => {
                    match handle_to_entity.map.get(&handle) {
                        Some(player_entity) => {
                            input_examine_entity.send(InputExamineEntity {
                                handle: handle,
                                examine_entity: Entity::from_bits(entity_id),
                                entity: *player_entity,
                                ..Default::default()
                            });
                        }
                        None => {
                            warn!("Couldn't find player_entity belonging to ExamineEntity sender handle.");
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetUnloadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub struct NetLoadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

use std::collections::HashMap;

use crate::entity_data::personalise;
use bevy::prelude::Transform;
use networking::server::EntityUpdateData;

use crate::entity_data::{EntityData, EntityUpdates};
/// Load an entity in for the client as a function.
#[cfg(feature = "server")]
pub fn load_entity(
    entity_updates: &HashMap<String, HashMap<String, EntityUpdateData>>,
    entity_transform: Transform,
    interpolated_transform: bool,
    net_load_entity: &mut EventWriter<NetLoadEntity>,
    player_handle: u64,
    entity_data: &EntityData,
    entity_updates_component: &EntityUpdates,
    entity_id: Entity,
    load_entirely: bool,
) {
    let mut hash_map;

    if load_entirely {
        hash_map = entity_updates.clone();

        personalise(&mut hash_map, player_handle, entity_updates_component);

        let transform_entity_update = EntityUpdateData::Transform(
            entity_transform.translation,
            entity_transform.rotation,
            entity_transform.scale,
        );

        match interpolated_transform {
            true => {
                let mut transform_hash_map = HashMap::new();
                transform_hash_map.insert("transform".to_string(), transform_entity_update);

                hash_map.insert("rawTransform".to_string(), transform_hash_map);
            }
            false => {
                let root_map_option = hash_map.get_mut(&".".to_string());

                match root_map_option {
                    Some(root_map) => {
                        root_map.insert("transform".to_string(), transform_entity_update);
                    }
                    None => {
                        let mut transform_hash_map = HashMap::new();
                        transform_hash_map.insert("transform".to_string(), transform_entity_update);

                        hash_map.insert(".".to_string(), transform_hash_map);
                    }
                }
            }
        }
    } else {
        hash_map = HashMap::new();
    }

    net_load_entity.send(NetLoadEntity {
        handle: player_handle,
        message: ReliableServerMessage::LoadEntity(
            entity_data.entity_class.clone(),
            entity_data.entity_name.clone(),
            hash_map,
            entity_id.to_bits(),
            load_entirely,
            "main".to_string(),
            "".to_string(),
            false,
        ),
    });
}

/// Unload an entity in for the client as a function.
#[cfg(feature = "server")]
pub fn unload_entity(
    player_handle: u64,
    entity_id: Entity,
    net_unload_entity: &mut EventWriter<NetUnloadEntity>,
    unload_entirely: bool,
) {
    net_unload_entity.send(NetUnloadEntity {
        handle: player_handle,
        message: ReliableServerMessage::UnloadEntity(entity_id.to_bits(), unload_entirely),
    });
}
