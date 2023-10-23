use bevy::prelude::{Event, Resource, SystemSet};

use crate::cache::PhysicsCache;

#[derive(Event, Clone, Resource, Default)]
pub struct StartCorrection {
    pub start_tick: u64,
    pub correction_id: u64,
}
#[derive(Resource, Default)]
pub struct CorrectionResource {
    pub id_iterative: u64,
}
#[derive(Event, Clone)]
pub struct CorrectionResults {
    pub correction_id: u64,
    pub data: PhysicsCache,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CorrectionSet {
    Start,
}
