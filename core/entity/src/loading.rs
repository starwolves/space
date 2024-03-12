use bevy::ecs::entity::Entity;
use bevy::ecs::system::Resource;
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
use crate::entity_types::EntityTypeCache;
use crate::entity_types::EntityTypes;
use crate::net::EntityServerMessage;
use crate::spawn::EntityBuildData;
use crate::spawn::PeerPawns;
use crate::spawn::ServerEntityClientEntity;
use crate::spawn::SpawnEntity;

#[derive(Resource, Default)]
pub struct NewToBeCachedSpawnedEntities {
    pub list: Vec<(u32, Entity)>,
}

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
    mut new: ResMut<NewToBeCachedSpawnedEntities>,
    mut type_cache: ResMut<EntityTypeCache>,
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
                let transform;
                match &load_entity.physics_data {
                    crate::net::PhysicsData::LoadData(data) => {
                        transform = Transform {
                            translation: data.translation,
                            rotation: data.rotation,
                            ..Default::default()
                        };
                    }
                    crate::net::PhysicsData::SpawnData(data) => {
                        transform = Transform {
                            translation: data.translation,
                            rotation: data.rotation,
                            ..Default::default()
                        };
                    }
                }

                let entity_default = T::default();

                if entity_default.is_type(identity.clone()) {
                    let c_id = commands.spawn(()).id();

                    map.map.insert(load_entity.entity, c_id);
                    info!(
                        "Spawning {} sid:{:?}, cid:{:?}, updates:{}.",
                        identity,
                        load_entity.entity,
                        c_id,
                        load_entity.entity_updates_reliable.len()
                            + load_entity.entity_updates_unreliable.len()
                    );

                    let etype;
                    match types.types.get(&identity) {
                        Some(t) => {
                            etype = t.clone();
                        }
                        None => {
                            warn!("Couldnt find entity type.");
                            continue;
                        }
                    }
                    type_cache.map.insert(c_id, etype);

                    /*info!(
                        "Loading pos {:?} for tick {} at tick {}",
                        transform.translation, server_tick, stamp.tick
                    );*/

                    spawn_events.send(SpawnEntity {
                        spawn_data: EntityBuildData {
                            entity_transform: transform,
                            correct_transform: false,
                            holder_entity_option: load_entity.holder_entity,
                            entity: Some(c_id),
                            server_entity: Some(load_entity.entity),
                            ..Default::default()
                        },
                        entity_type: entity_default,
                    });

                    if load_entity.entity_updates_reliable.len() > 0 {
                        queue.reliable.insert(
                            c_id,
                            (message.stamp, load_entity.entity_updates_reliable.clone()),
                        );
                    }
                    if load_entity.entity_updates_unreliable.len() > 0 {
                        queue.unreliable.insert(
                            c_id,
                            (message.stamp, load_entity.entity_updates_unreliable.clone()),
                        );
                    }

                    let compare_tick;
                    let mut adjusted_tick = message.stamp - 1;

                    match &load_entity.physics_data {
                        crate::net::PhysicsData::LoadData(_) => {
                            compare_tick = message.stamp;
                        }
                        crate::net::PhysicsData::SpawnData(_) => {
                            compare_tick = message.stamp - 1;
                            adjusted_tick -= 1;
                        }
                    }

                    if compare_tick != stamp.tick {
                        new.list.push((compare_tick, c_id));
                        correction.send(StartCorrection {
                            start_tick: adjusted_tick,
                            last_tick: stamp.tick,
                        });
                    } else {
                        info!("Perfect load entity sync.");
                    }
                    /*match priority.cache.get_mut(&compare_tick) {
                        Some(priority_cache) => {
                            priority_cache.insert(c_id, priority_update);
                        }
                        None => {
                            let mut map = HashMap::new();
                            map.insert(c_id, priority_update);
                            priority.cache.insert(compare_tick, map);
                        }
                    }*/
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
