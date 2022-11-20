use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::Query;
use bevy::prelude::Res;
use networking::server::HandleToEntity;
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;

use crate::sensable::Sensable;

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
