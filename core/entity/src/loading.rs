use bevy::log::info;
use bevy::log::warn;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::ResMut;
use bevy::prelude::Transform;
use bevy_renet::renet::ClientId;
use networking::client::IncomingReliableServerMessage;

use bevy::prelude::Res;
use networking::stamp::TickRateStamp;
use resources::correction::StartCorrection;

use crate::entity_data::QueuedSpawnEntityUpdates;
use crate::entity_types::EntityType;
use crate::entity_types::EntityTypes;
use crate::net::EntityServerMessage;
use crate::spawn::EntityBuildData;
use crate::spawn::PeerPawns;
use crate::spawn::ServerEntityClientEntity;
use crate::spawn::SpawnEntity;

/// Client loads in entities.
pub fn load_entity<T: Send + Sync + 'static + Default + EntityType>(
    mut client: EventReader<IncomingReliableServerMessage<EntityServerMessage>>,
    types: Res<EntityTypes>,
    mut spawn_events: EventWriter<SpawnEntity<T>>,
    mut commands: Commands,
    mut map: ResMut<ServerEntityClientEntity>,
    mut correction: EventWriter<StartCorrection>,
    stamp: Res<TickRateStamp>,
    mut queue: ResMut<QueuedSpawnEntityUpdates>,
) {
    for message in client.read() {
        match &message.message {
            EntityServerMessage::LoadEntity(load_entity) => {
                let index = types
                    .netcode_types
                    .values()
                    .position(|r| r == &load_entity.type_id)
                    .unwrap();
                let keys: Vec<&String> = types.netcode_types.keys().collect();
                let identity;
                match keys.get(index) {
                    Some(i) => identity = i.to_string(),
                    None => {
                        warn!("Coudlnt find entity type in map.");
                        continue;
                    }
                }

                let transform = Transform {
                    translation: load_entity.physics_data.translation,
                    scale: load_entity.physics_data.scale,
                    rotation: load_entity.physics_data.rotation,
                };

                let entity_default = T::default();

                if entity_default.is_type(identity.clone()) {
                    let c_id = commands.spawn(()).id();

                    map.map.insert(load_entity.entity, c_id);
                    info!(
                        "Spawning {} sid:{:?}, cid:{:?}, updates:{}",
                        identity,
                        load_entity.entity,
                        c_id,
                        load_entity.entity_updates_reliable.len()
                            + load_entity.entity_updates_unreliable.len()
                    );

                    spawn_events.send(SpawnEntity {
                        spawn_data: EntityBuildData {
                            entity_transform: transform,
                            correct_transform: false,
                            holder_entity_option: load_entity.holder_entity,
                            entity: c_id,
                            server_entity: Some(load_entity.entity),
                            ..Default::default()
                        },
                        entity_type: entity_default,
                    });

                    if load_entity.entity_updates_reliable.len() > 0 {
                        queue
                            .reliable
                            .insert(c_id, load_entity.entity_updates_reliable.clone());
                    }
                    if load_entity.entity_updates_unreliable.len() > 0 {
                        queue
                            .unreliable
                            .insert(c_id, load_entity.entity_updates_unreliable.clone());
                    }

                    queue.stamp = message.stamp;
                    let large = stamp.calculate_large(message.stamp);
                    correction.send(StartCorrection {
                        start_tick: large,
                        last_tick: stamp.large,
                    });
                }
            }
            _ => {}
        }
    }
}

pub(crate) fn link_peer(
    mut client: EventReader<IncomingReliableServerMessage<EntityServerMessage>>,
    mut peers: ResMut<PeerPawns>,
    links: Res<ServerEntityClientEntity>,
) {
    for message in client.read() {
        match &message.message {
            EntityServerMessage::LinkPeer(link) => {
                let mut found = false;
                for (s, c) in links.map.iter() {
                    if s == &link.server_entity {
                        peers.map.insert(ClientId::from_raw(link.handle.into()), *c);
                        found = true;
                        break;
                    }
                }
                if !found {
                    warn!(
                        "Couldnt find link peer server entity. {:?}",
                        link.server_entity
                    );
                }
            }
            _ => (),
        }
    }
}
