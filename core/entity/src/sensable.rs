use bevy::ecs::entity::Entity;
use bevy::prelude::{Commands, Component, EventReader, EventWriter, Query, Res};
use networking::server::{HandleToEntity, ReliableServerMessage};
use networking_macros::NetMessage;

/// The component for entities that can be sensed.
#[derive(Component, Default)]
#[cfg(feature = "server")]
pub struct Sensable {
    pub is_light: bool,
    pub is_audible: bool,
    pub sensed_by: Vec<Entity>,
    pub sensed_by_cached: Vec<Entity>,
    pub always_sensed: bool,
}

use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetDespawnEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

///Despawn sensable component event.
pub struct DespawnEntity {
    pub entity: Entity,
}

/// Executes despawn logic for Sensable components.
/// Shouldn't be called from the same stage visible_checker.system() runs in.
pub(crate) fn despawn_entity(
    mut despawn_event: EventReader<DespawnEntity>,
    mut net_unload_entity: EventWriter<NetDespawnEntity>,
    handle_to_entity: Res<HandleToEntity>,
    mut sensable_query: Query<&mut Sensable>,
    mut commands: Commands,
) {
    for event in despawn_event.iter() {
        match sensable_query.get_mut(event.entity) {
            Ok(mut sensable_component) => {
                for sensed_by_entity in sensable_component.sensed_by.iter() {
                    match handle_to_entity.inv_map.get(&sensed_by_entity) {
                        Some(handle) => {
                            net_unload_entity.send(NetDespawnEntity {
                                handle: *handle,
                                message: ReliableServerMessage::UnloadEntity(
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
                            net_unload_entity.send(NetDespawnEntity {
                                handle: *handle,
                                message: ReliableServerMessage::UnloadEntity(
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
