use bevy::ecs::system::ResMut;
use bevy::prelude::Commands;
use bevy::prelude::Entity;
use bevy::prelude::Event;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy_renet::renet::ClientId;
use networking::server::ConstructEntityUpdates;
use networking::server::HandleToEntity;

use crate::net::EntityServerMessage;
use crate::sensable::Sensable;

///Despawn sensable component event.
#[derive(Event)]
pub struct DespawnClientEntity {
    pub entity: Entity,
}
use networking::server::OutgoingReliableServerMessage;

/// Event to load in entity for client.
#[derive(Event)]
pub struct SpawnClientEntity {
    pub entity: Entity,
    pub loader_handle: ClientId,
}

pub(crate) fn construct_entity_updates(
    mut send: ResMut<ConstructEntityUpdates>,
    mut events: EventReader<SpawnClientEntity>,
) {
    for e in events.read() {
        send.entities.insert(e.entity, true);
    }
}

/// Executes despawn logic for Sensable components.
/// Shouldn't be called from the same stage visible_checker.system() runs in.

pub(crate) fn despawn_entity(
    mut despawn_event: EventReader<DespawnClientEntity>,
    handle_to_entity: Res<HandleToEntity>,
    mut sensable_query: Query<&mut Sensable>,
    mut commands: Commands,
    mut net: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
) {
    for event in despawn_event.read() {
        match sensable_query.get_mut(event.entity) {
            Ok(mut sensable_component) => {
                for sensed_by_entity in sensable_component.sensed_by.iter() {
                    match handle_to_entity.inv_map.get(&sensed_by_entity) {
                        Some(handle) => {
                            net.send(OutgoingReliableServerMessage {
                                handle: *handle,
                                message: EntityServerMessage::UnloadEntity(event.entity),
                            });
                        }
                        None => {}
                    }
                }

                sensable_component.sensed_by = vec![];
            }
            Err(_) => {}
        }

        commands.entity(event.entity).despawn();
    }
}
