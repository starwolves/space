use bevy::ecs::system::Resource;

use crate::cache::PhysicsCache;

#[derive(Resource, Clone, Default)]
pub struct CorrectionResults {
    pub data: PhysicsCache,
}
