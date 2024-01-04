use bevy::{
    ecs::{event::EventWriter, system::Res},
    log::{info, warn},
    prelude::{Commands, DespawnRecursiveExt, Entity, Event, EventReader},
};
use networking::client::IncomingReliableServerMessage;

use crate::{net::EntityServerMessage, spawn::ServerEntityClientEntity};

/// The event to use to despawn an entity.
#[derive(Event)]
pub struct DespawnEntity {
    pub entity: Entity,
}

pub(crate) fn despawn_entities(mut events: EventReader<DespawnEntity>, mut commands: Commands) {
    for event in events.read() {
        commands.entity(event.entity).despawn_recursive();
    }
}

pub(crate) fn client_despawn_entity(
    mut net: EventReader<IncomingReliableServerMessage<EntityServerMessage>>,
    mut despawn: EventWriter<DespawnEntity>,
    links: Res<ServerEntityClientEntity>,
) {
    for message in net.read() {
        match message.message {
            EntityServerMessage::UnloadEntity(entity) => match links.map.get(&entity) {
                Some(client_entity) => {
                    despawn.send(DespawnEntity {
                        entity: *client_entity,
                    });
                    info!("Despawning {:?}.", client_entity);
                }
                None => {
                    warn!("Couldnt find client entity for server entity: {:?}", entity);
                }
            },
            _ => (),
        }
    }
}
