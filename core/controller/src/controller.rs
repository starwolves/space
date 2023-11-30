use std::collections::HashMap;

use bevy::{
    ecs::system::{Query, Res, ResMut, Resource},
    prelude::{Component, Entity, Vec2},
};
use cameras::LookTransform;
use networking::{
    server::{ConnectedPlayer, ConstructEntityUpdates, EntityUpdates},
    stamp::TickRateStamp,
};
use pawn::net::UnreliableControllerClientMessage;
use serde::{Deserialize, Serialize};

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
    pub cache: HashMap<u64, HashMap<Entity, ControllerInput>>,
}

pub(crate) fn cache_controller(
    query: Query<(Entity, &ControllerInput)>,
    stamp: Res<TickRateStamp>,
    mut cache: ResMut<ControllerCache>,
) {
    for (entity, controller) in query.iter() {
        match cache.cache.get_mut(&stamp.large) {
            Some(map) => {
                map.insert(entity, controller.clone());
            }
            None => {
                let mut map = HashMap::new();
                map.insert(entity, controller.clone());
                cache.cache.insert(stamp.large, map);
            }
        }
    }

    // Clean cache.
    let mut to_remove = vec![];
    for recorded_stamp in cache.cache.keys() {
        if stamp.large >= 256 && recorded_stamp < &(stamp.large - 256) {
            to_remove.push(*recorded_stamp);
        }
    }
    for i in to_remove {
        cache.cache.remove(&i);
    }
}

pub(crate) fn look_transform_entity_update(
    mut updates: ResMut<EntityUpdates<PeerUnreliableControllerMessage>>,
    query: Query<(Entity, &LookTransform, &ConnectedPlayer)>,
    construct: Res<ConstructEntityUpdates>,
    stamp: Res<TickRateStamp>,
) {
    for (c, _) in construct.entities.iter() {
        match query.get(*c) {
            Ok((entity, look_transform, connected_player)) => {
                updates.map.insert(
                    entity,
                    vec![PeerUnreliableControllerMessage {
                        message: UnreliableControllerClientMessage::SyncLookTransform(
                            look_transform.target,
                        ),
                        peer_handle: connected_player.handle.raw() as u16,
                        client_stamp: stamp.tick,
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
    stamp: Res<TickRateStamp>,
) {
    for (c, _) in construct.entities.iter() {
        match query.get(*c) {
            Ok((entity, controller_input, connected_player)) => {
                updates.map.insert(
                    entity,
                    vec![PeerReliableControllerMessage {
                        message: ControllerClientMessage::SyncControllerInput(
                            controller_input.clone(),
                        ),
                        peer_handle: connected_player.handle.raw() as u16,
                        client_stamp: stamp.tick,
                    }],
                );
            }
            Err(_) => {}
        }
    }
}

use crate::{
    net::ControllerClientMessage,
    networking::{PeerReliableControllerMessage, PeerUnreliableControllerMessage},
};
