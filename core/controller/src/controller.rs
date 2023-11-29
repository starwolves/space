use std::collections::HashMap;

use bevy::{
    ecs::system::{Query, Res, ResMut, Resource},
    prelude::{Component, Entity, Vec2},
};
use cameras::LookTransform;
use networking::{server::EntityUpdates, stamp::TickRateStamp};
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
    query: Query<(Entity, &LookTransform)>,
) {
    for (entity, look_transform) in query.iter() {
        updates.map.insert(
            entity,
            vec![PeerUnreliableControllerMessage {
                message: UnreliableControllerClientMessage::SyncLookTransform(
                    look_transform.target,
                ),
                peer_handle: 0,
                client_stamp: 0,
            }],
        );
    }
}

pub(crate) fn controller_input_entity_update(
    mut updates: ResMut<EntityUpdates<PeerReliableControllerMessage>>,
    query: Query<(Entity, &ControllerInput)>,
) {
    for (entity, controller_input) in query.iter() {
        updates.map.insert(
            entity,
            vec![PeerReliableControllerMessage {
                message: ControllerClientMessage::ControllerSync(controller_input.clone()),
                peer_handle: 0,
                client_stamp: 0,
            }],
        );
    }
}

use crate::{
    net::ControllerClientMessage,
    networking::{PeerReliableControllerMessage, PeerUnreliableControllerMessage},
};
