use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::EventReader;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy_renet::renet::RenetServer;
use networking::plugin::RENET_RELIABLE_CHANNEL_ID;
use networking::server::HandleToEntity;

use crate::networking::EntityServerMessage;
use crate::sensable::Sensable;

///Despawn sensable component event.
pub struct DespawnEntity {
    pub entity: Entity,
}

/// Executes despawn logic for Sensable components.
/// Shouldn't be called from the same stage visible_checker.system() runs in.
pub(crate) fn despawn_entity(
    mut despawn_event: EventReader<DespawnEntity>,
    handle_to_entity: Res<HandleToEntity>,
    mut sensable_query: Query<&mut Sensable>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
) {
    for event in despawn_event.iter() {
        match sensable_query.get_mut(event.entity) {
            Ok(mut sensable_component) => {
                for sensed_by_entity in sensable_component.sensed_by.iter() {
                    match handle_to_entity.inv_map.get(&sensed_by_entity) {
                        Some(handle) => {
                            server.send_message(
                                *handle,
                                RENET_RELIABLE_CHANNEL_ID,
                                bincode::serialize(&EntityServerMessage::UnloadEntity(
                                    event.entity.to_bits(),
                                    true,
                                ))
                                .unwrap(),
                            );
                        }
                        None => {}
                    }
                }
                for sensed_by_entity in sensable_component.sensed_by_cached.iter() {
                    match handle_to_entity.inv_map.get(&sensed_by_entity) {
                        Some(handle) => {
                            server.send_message(
                                *handle,
                                RENET_RELIABLE_CHANNEL_ID,
                                bincode::serialize(&EntityServerMessage::UnloadEntity(
                                    event.entity.to_bits(),
                                    true,
                                ))
                                .unwrap(),
                            );
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
