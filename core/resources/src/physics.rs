use std::collections::HashMap;

use bevy::{
    ecs::{entity::Entity, schedule::SystemSet, system::Resource},
    math::{Quat, Vec3},
};
use serde::{Deserialize, Serialize};

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PhysicsSet {
    Correct,
    Cache,
    CacheNewSpawns,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmallCache {
    pub entity: Entity,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    pub translation: Vec3,
    pub rotation: Quat,
}

/// Contains known authorative physics data.
#[derive(Resource, Default, Clone)]
pub struct PriorityPhysicsCache {
    pub cache: HashMap<u64, HashMap<Entity, PriorityUpdate>>,
}
#[derive(Clone, Debug)]
pub enum PriorityUpdate {
    SmallCache(SmallCache),
    Position(Vec3),
    PhysicsSpawn(PhysicsSpawn),
}

/// Currently only support spawning bodies with transform parameter. In the future other inits like velocity will be added.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsSpawn {
    pub translation: Vec3,
    pub rotation: Quat,
}
