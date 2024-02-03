use std::collections::HashMap;

use bevy::ecs::entity::Entity;
use bevy::ecs::system::{ResMut, Resource};
use bevy::log::warn;
use bevy::prelude::{Component, Event, SystemSet, Transform};
use entity_macros::Identity;
use networking::client::{
    IncomingRawReliableServerMessage, IncomingRawUnreliableServerMessage, QueuedSpawnEntityRaw,
};
use networking::messaging::{
    ReliableMessage, ReliableServerMessageBatch, UnreliableMessage, UnreliableServerMessageBatch,
};
use networking::stamp::TickRateStamp;
use serde::{Deserialize, Serialize};

use crate::entity_types::{BoxedEntityType, EntityType};
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum EntityWorldType {
    Main,
    HealthUI,
}

use crate::init::RawEntityRon;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InterpolationSet {
    Main,
}

/// Component for entities that were included and spawned with the map itself.
#[derive(Component)]

pub struct DefaultMapEntity;

/// Event about spawning entities from ron.
#[derive(Event)]
pub struct RawSpawnEvent {
    pub raw_entity: RawEntityRon,
}
/// ron entity.
#[derive(Deserialize, Clone)]

pub struct RawEntity {
    pub entity_type: String,
    pub transform: String,
    pub data: String,
}

/// Component with the cache of the latest broadcasted transforms for its entity.
#[derive(Component, Default)]

pub struct CachedBroadcastTransform {
    pub transform: Transform,
    pub is_active: bool,
}
/// Component with transform for sound effects.
#[derive(Component)]

pub struct UpdateTransform;
/// The base entity component holding base entity data.
#[derive(Component)]

pub struct EntityData {
    pub entity_type: BoxedEntityType,
    pub entity_group: EntityGroup,
}
#[derive(Clone, Identity)]
pub struct BlankEntityType {
    pub identifier: String,
}
impl Default for BlankEntityType {
    fn default() -> Self {
        Self {
            identifier: "Blank".to_string(),
        }
    }
}

#[derive(Copy, Clone, Default)]

pub enum EntityGroup {
    #[default]
    None,
    AirLock,
    CounterWindowSensor,
    Pawn,
}

/// World mode component.
#[derive(Component)]

pub struct WorldMode {
    pub mode: WorldModes,
}

/// All world modes.
#[derive(Debug)]

pub enum WorldModes {
    Static,
    Kinematic,
    Physics,
    Held,
    Worn,
}
/// For entities that are also registered with the gridmap.

pub struct GridItemData {
    pub transform_offset: Transform,
    /// So this entity can be built on a cell when another item is already present on that cell.
    pub can_be_built_with_grid_item: Vec<String>,
}

pub trait GridEntity {
    fn get_grid_item_data() -> GridItemData;
}

#[derive(Resource, Default)]
pub struct QueuedSpawnEntityUpdates {
    pub reliable: HashMap<Entity, (u64, Vec<Vec<u8>>)>,
    pub unreliable: HashMap<Entity, (u64, Vec<Vec<u8>>)>,
}

/// We have to deserialize raw messages for a second time that are inside the LoadEntity call as it supplies an additional collection of Entity Updates
/// that are only detectable after the first raw serialization run.
pub(crate) fn fire_load_entity_updates(
    mut queue: ResMut<QueuedSpawnEntityUpdates>,
    mut raw: ResMut<QueuedSpawnEntityRaw>,
) {
    for (_, (large, updates)) in queue.reliable.drain() {
        let mut msgs = vec![];
        for update in updates {
            match bincode::deserialize::<ReliableMessage>(&update) {
                Ok(msg) => {
                    msgs.push(msg);
                }
                Err(_) => {
                    warn!("Couldnt deserialize ReliableMessage");
                }
            }
        }

        raw.reliable.push(IncomingRawReliableServerMessage {
            message: ReliableServerMessageBatch {
                messages: msgs,
                stamp: TickRateStamp::new(large).tick,
            },
        });
    }

    for (_, (large, updates)) in queue.unreliable.drain() {
        let mut msgs = vec![];
        for update in updates {
            match bincode::deserialize::<UnreliableMessage>(&update) {
                Ok(msg) => {
                    msgs.push(msg);
                }
                Err(_) => {
                    warn!("Couldnt deserialize UnreliableMessage");
                }
            }
        }

        raw.unreliable.push(IncomingRawUnreliableServerMessage {
            message: UnreliableServerMessageBatch {
                messages: msgs,
                stamp: TickRateStamp::new(large).tick,
            },
        });
    }
}
