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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmallCache {
    pub entity: Entity,
    pub linear_velocity: Vec3,
    pub angular_velocity: Vec3,
    pub translation: Vec3,
    pub rotation: Quat,
}

#[derive(Resource, Default, Clone)]
pub struct PriorityPhysicsCache {
    pub cache: HashMap<u64, HashMap<Entity, PriorityUpdate>>,
}
#[derive(Clone)]
pub enum PriorityUpdate {
    SmallCache(SmallCache),
    Position(Vec3),
}
