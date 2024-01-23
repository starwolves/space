use std::collections::HashMap;

use bevy::{
    ecs::system::{Query, Res, ResMut, Resource},
    prelude::{Component, Entity, Vec2},
    transform::components::Transform,
};
use cameras::LookTransform;
use itertools::Itertools;
use networking::{
    server::{ConnectedPlayer, ConstructEntityUpdates, EntityUpdates},
    stamp::TickRateStamp,
};
use pawn::net::{PeerUpdateLookTransform, UnreliablePeerControllerClientMessage};
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
    pub cache: HashMap<Entity, HashMap<u64, ControllerInput>>,
}

pub(crate) fn clean_controller_cache(mut cache: ResMut<ControllerCache>) {
    // Clean cache.
    for (_, cache) in cache.cache.iter_mut() {
        if cache.len() > MAX_CACHE_TICKS_AMNT as usize {
            let mut j = 0;

            for i in cache.clone().keys().sorted().rev() {
                if j as usize == cache.len() - MAX_CACHE_TICKS_AMNT as usize {
                    continue;
                }
                if j >= MAX_CACHE_TICKS_AMNT {
                    cache.remove(i);
                }

                j += 1;
            }
        }
    }
}

pub(crate) fn look_transform_entity_update(
    mut updates: ResMut<EntityUpdates<PeerUnreliableControllerMessage>>,
    query: Query<(Entity, &LookTransform, &ConnectedPlayer, &Transform)>,
    construct: Res<ConstructEntityUpdates>,
    stamp: Res<TickRateStamp>,
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
                        message: PeerControllerClientMessage::SyncControllerInput(
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
