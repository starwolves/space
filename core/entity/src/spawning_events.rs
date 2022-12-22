use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::Query;
use bevy::prelude::Res;
use networking::server::HandleToEntity;

use crate::net::EntityServerMessage;
use crate::sensable::Sensable;

///Despawn sensable component event.
#[cfg(feature = "server")]
pub struct DespawnClientEntity {
    pub entity: Entity,
}
use networking::server::OutgoingReliableServerMessage;

/// Event to load in entity for client.
pub struct SpawnClientEntity {
    pub entity: Entity,
    pub loader_handle: u64,
    pub load_entirely: bool,
}
/// Executes despawn logic for Sensable components.
/// Shouldn't be called from the same stage visible_checker.system() runs in.
#[cfg(feature = "server")]
pub(crate) fn despawn_entity(
    mut despawn_event: EventReader<DespawnClientEntity>,
    handle_to_entity: Res<HandleToEntity>,
    mut sensable_query: Query<&mut Sensable>,
    mut commands: Commands,
    mut net: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
) {
    for event in despawn_event.iter() {
        match sensable_query.get_mut(event.entity) {
            Ok(mut sensable_component) => {
                for sensed_by_entity in sensable_component.sensed_by.iter() {
                    match handle_to_entity.inv_map.get(&sensed_by_entity) {
                        Some(handle) => {
                            net.send(OutgoingReliableServerMessage {
                                handle: *handle,
                                message: EntityServerMessage::UnloadEntity(
                                    event.entity.to_bits(),
                                    true,
                                ),
                            });
                        }
                        None => {}
                    }
                }
                for sensed_by_entity in sensable_component.sensed_by_cached.iter() {
                    match handle_to_entity.inv_map.get(&sensed_by_entity) {
                        Some(handle) => {
                            net.send(OutgoingReliableServerMessage {
                                handle: *handle,
                                message: EntityServerMessage::UnloadEntity(
                                    event.entity.to_bits(),
                                    true,
                                ),
                            });
                        }
                        None => {}
                    }
                }

                sensable_component.sensed_by = vec![];
                sensable_component.sensed_by_cached = vec![];
            }
            Err(_) => {}
        }

        commands.entity(event.entity).despawn();
    }
}

use crate::entity_data::personalise;
use crate::entity_data::WorldModes;
use bevy::prelude::warn;
use bevy::prelude::Transform;
use bevy_rapier3d::prelude::RigidBody;

use std::collections::HashMap;

use crate::entity_data::{EntityData, EntityUpdates, WorldMode};
use networking::server::EntityUpdateData;
/// Load an entity in for the client.
#[cfg(feature = "server")]
pub(crate) fn spawn_entity_for_client(
    entity_query: Query<(
        &EntityData,
        &EntityUpdates,
        &Transform,
        Option<&RigidBody>,
        Option<&WorldMode>,
    )>,
    mut load_entity_events: EventReader<SpawnClientEntity>,
    mut server: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
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

                server.send(OutgoingReliableServerMessage {
                    handle: load_entity_event.loader_handle,
                    message: EntityServerMessage::LoadEntity(
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
