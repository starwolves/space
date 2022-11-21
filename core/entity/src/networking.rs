use bevy::prelude::ResMut;

use bevy::prelude::warn;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;
use serde::Deserialize;
use serde::Serialize;

use crate::examine::InputExamineEntity;
use bevy::prelude::Entity;
use bevy::prelude::EventWriter;
use bevy::prelude::Res;
use networking::server::HandleToEntity;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub enum EntityMessage {
    ExamineEntity(u64),
}

/// Manage incoming network messages from clients.
#[cfg(feature = "server")]
pub(crate) fn incoming_messages(
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
    mut input_examine_entity: EventWriter<InputExamineEntity>,
) {
    for handle in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(handle, RENET_RELIABLE_CHANNEL_ID) {
            let client_message_result: Result<EntityMessage, _> = bincode::deserialize(&message);
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
                EntityMessage::ExamineEntity(entity_id) => {
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
            }
        }
    }
}
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetUnloadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetLoadEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

use std::collections::HashMap;

use crate::entity_data::personalise;
use crate::entity_data::EntityUpdates;
use bevy::prelude::Query;
use bevy::prelude::Transform;
use networking::server::EntityUpdateData;

use crate::entity_data::EntityData;

/// Event to load in entity for client.
pub struct LoadEntity {
    pub entity: Entity,
    pub loader_handle: u64,
    pub load_entirely: bool,
}
use crate::entity_data::WorldMode;
use crate::entity_data::WorldModes;
use bevy::prelude::EventReader;
use bevy_rapier3d::prelude::RigidBody;

/// Load an entity in for the client.
#[cfg(feature = "server")]
pub(crate) fn load_entity(
    entity_query: Query<(
        &EntityData,
        &EntityUpdates,
        &Transform,
        Option<&RigidBody>,
        Option<&WorldMode>,
    )>,
    mut load_entity_events: EventReader<LoadEntity>,
    mut net_load_entity: EventWriter<NetLoadEntity>,
) {
    for load_entity_event in load_entity_events.iter() {
        match entity_query.get(load_entity_event.entity) {
            Ok((
                entity_data,
                entity_update,
                transform,
                rigid_body_component_option,
                entity_world_mode_option,
            )) => {
                let mut is_interpolated = false;

                match rigid_body_component_option {
                    Some(rigid_body_component) => match rigid_body_component {
                        RigidBody::Dynamic => match entity_world_mode_option {
                            Some(entity_world_mode) => {
                                if matches!(entity_world_mode.mode, WorldModes::Held)
                                    || matches!(entity_world_mode.mode, WorldModes::Worn)
                                {
                                    is_interpolated = false;
                                } else {
                                    is_interpolated = true;
                                }
                            }
                            None => {
                                is_interpolated = false;
                            }
                        },
                        RigidBody::Fixed => {}
                        _ => {
                            warn!("Unexpected rigidbody type.");
                            continue;
                        }
                    },
                    None => {}
                }

                let mut hash_map;

                if load_entity_event.load_entirely {
                    hash_map = entity_update.updates.clone();

                    personalise(
                        &mut hash_map,
                        load_entity_event.loader_handle,
                        entity_update,
                    );

                    let transform_entity_update = EntityUpdateData::Transform(
                        transform.translation,
                        transform.rotation,
                        transform.scale,
                    );

                    match is_interpolated {
                        true => {
                            let mut transform_hash_map = HashMap::new();
                            transform_hash_map
                                .insert("transform".to_string(), transform_entity_update);

                            hash_map.insert("rawTransform".to_string(), transform_hash_map);
                        }
                        false => {
                            let root_map_option = hash_map.get_mut(&".".to_string());

                            match root_map_option {
                                Some(root_map) => {
                                    root_map
                                        .insert("transform".to_string(), transform_entity_update);
                                }
                                None => {
                                    let mut transform_hash_map = HashMap::new();
                                    transform_hash_map
                                        .insert("transform".to_string(), transform_entity_update);

                                    hash_map.insert(".".to_string(), transform_hash_map);
                                }
                            }
                        }
                    }
                } else {
                    hash_map = HashMap::new();
                }

                net_load_entity.send(NetLoadEntity {
                    handle: load_entity_event.loader_handle,
                    message: ReliableServerMessage::LoadEntity(
                        entity_data.entity_class.clone(),
                        entity_data.entity_name.clone(),
                        hash_map,
                        load_entity_event.entity.to_bits(),
                        load_entity_event.load_entirely,
                        "main".to_string(),
                        "".to_string(),
                        false,
                    ),
                });
            }
            Err(_) => {
                warn!("Couldnt find entity for load entity event.");
            }
        }
    }
}
