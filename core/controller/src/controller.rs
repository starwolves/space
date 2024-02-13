use std::collections::{BTreeMap, HashMap};

use bevy::{
    ecs::{
        event::EventReader,
        system::{Local, Query, Res, ResMut, Resource},
    },
    log::warn,
    math::Vec3,
    prelude::{Component, Entity, Vec2},
    transform::components::Transform,
};
use bevy_renet::renet::ClientId;
use cameras::LookTransform;
use networking::{
    server::{
        ConnectedPlayer, ConstructEntityUpdates, EntityUpdates, HandleToEntity,
        IncomingUnreliableClientMessage,
    },
    stamp::TickRateStamp,
};
use pawn::net::{
    PeerUpdateLookTransform, UnreliableControllerClientMessage,
    UnreliablePeerControllerClientMessage,
};
use resources::correction::MAX_CACHE_TICKS_AMNT;
use serde::{Deserialize, Serialize};

use crate::{
    net::PeerControllerClientMessage,
    networking::{PeerReliableControllerMessage, PeerUnreliableControllerMessage},
};

/// Controller input component.
#[derive(Component, Clone, Debug, Serialize, Deserialize)]

pub struct ControllerInput {
    pub movement_vector: Vec2,
}
impl Default for ControllerInput {
    fn default() -> Self {
        Self {
            movement_vector: Vec2::ZERO,
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct ControllerCache {
    pub cache: HashMap<Entity, BTreeMap<u32, ControllerInput>>,
}

pub(crate) fn clean_controller_cache(mut cache: ResMut<ControllerCache>) {
    // Clean cache.
    for (_, cache) in cache.cache.iter_mut() {
        if cache.len() > MAX_CACHE_TICKS_AMNT as usize {
            let mut j = 0;
            let mut is = vec![];

            for i in cache.keys().rev() {
                if j >= MAX_CACHE_TICKS_AMNT {
                    is.push(*i);
                }
                j += 1;
            }
            for i in is {
                cache.remove(&i);
            }
        }
    }
}

pub(crate) fn look_transform_entity_update(
    mut updates: ResMut<EntityUpdates<PeerUnreliableControllerMessage>>,
    query: Query<(Entity, &LookTransform, &ConnectedPlayer, &Transform)>,
    construct: Res<ConstructEntityUpdates>,
) {
    for (c, _) in construct.entities.iter() {
        match query.get(*c) {
            Ok((entity, look_transform, connected_player, transform)) => {
                updates.map.insert(
                    entity,
                    vec![PeerUnreliableControllerMessage {
                        message: UnreliablePeerControllerClientMessage::UpdateLookTransform(
                            PeerUpdateLookTransform {
                                position: transform.translation,
                                target: look_transform.target,
                                sub_tick: u8::MAX,
                            },
                        ),
                        peer_handle: connected_player.handle.raw() as u16,
                    }],
                );
            }
            Err(_) => {}
        }
    }
}

pub(crate) fn controller_input_entity_update(
    mut updates: ResMut<EntityUpdates<PeerReliableControllerMessage>>,
    query: Query<(Entity, &ControllerInput, &ConnectedPlayer)>,
    construct: Res<ConstructEntityUpdates>,
) {
    for (c, _) in construct.entities.iter() {
        match query.get(*c) {
            Ok((entity, controller_input, connected_player)) => {
                updates.map.insert(
                    entity,
                    vec![PeerReliableControllerMessage {
                        message: PeerControllerClientMessage::SyncControllerInput(
                            controller_input.clone(),
                        ),
                        peer_handle: connected_player.handle.raw() as u16,
                    }],
                );
            }
            Err(_) => {}
        }
    }
}

pub(crate) fn server_sync_look_transform(
    mut humanoids: Query<&mut LookTransform>,
    mut messages: EventReader<IncomingUnreliableClientMessage<UnreliableControllerClientMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut queue: Local<HashMap<ClientId, BTreeMap<u32, BTreeMap<u8, Vec3>>>>,
    stamp: Res<TickRateStamp>,
) {
    for msg in messages.read() {
        match msg.message {
            UnreliableControllerClientMessage::UpdateLookTransform(target, id) => {
                match queue.get_mut(&msg.handle) {
                    Some(q1) => match q1.get_mut(&msg.stamp) {
                        Some(q2) => {
                            q2.insert(id, target);
                        }
                        None => {
                            let mut m = BTreeMap::new();
                            m.insert(id, target);
                            q1.insert(msg.stamp, m);
                        }
                    },
                    None => {
                        let mut n = BTreeMap::new();
                        n.insert(id, target);
                        let mut m = BTreeMap::new();
                        m.insert(msg.stamp, n);
                        queue.insert(msg.handle, m);
                    }
                }
            }
        }
    }
    let mut old_handles = vec![];
    for (handle, q) in queue.iter() {
        for i in q.keys().rev() {
            if i > &stamp.tick {
                continue;
            }
            let q2 = q.get(i).unwrap();
            for sub in q2.keys().rev() {
                let target = *q2.get(sub).unwrap();

                match handle_to_entity.map.get(&handle) {
                    Some(entity) => match humanoids.get_mut(*entity) {
                        Ok(mut look_transform) => {
                            look_transform.target = target;
                        }
                        Err(_) => {
                            warn!("Couldnt find client entity components.");
                        }
                    },
                    None => {
                        old_handles.push(*handle);
                    }
                }
                break;
            }
            break;
        }
    }
    for handle in old_handles {
        queue.remove(&handle);
    }

    // Clean cache.
    for (_, cache) in queue.iter_mut() {
        if cache.len() > MAX_CACHE_TICKS_AMNT as usize {
            let mut j = 0;
            let mut is = vec![];

            for i in cache.keys().rev() {
                if j >= MAX_CACHE_TICKS_AMNT {
                    is.push(*i);
                }
                j += 1;
            }
            for i in is {
                cache.remove(&i);
            }
        }
    }
}
