use bevy::prelude::{Event, Resource, SystemSet};

use crate::cache::PhysicsCache;

#[derive(Event, Clone, Resource, Default)]
pub struct StartCorrection {
    pub start_tick: u64,
}
#[derive(Event, Clone)]
pub struct CorrectionResults {
    pub data: PhysicsCache,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CorrectionSet {
    Start,
}
