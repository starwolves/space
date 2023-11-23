use std::collections::HashMap;

use bevy::{
    ecs::system::{Query, Res, ResMut, Resource},
    prelude::{Component, Entity, Vec2},
};
use networking::stamp::TickRateStamp;

/// Controller input component.
#[derive(Component, Clone, Debug)]

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
