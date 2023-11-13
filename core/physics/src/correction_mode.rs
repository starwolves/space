use bevy::prelude::{Event, Resource, SystemSet};

use crate::cache::Cache;

#[derive(Event, Clone, Resource, Default)]
pub struct StartCorrection {
    pub start_tick: u64,
    pub last_tick: u64,
}
#[derive(Event, Clone)]
pub struct CorrectionResults {
    pub data: Vec<Cache>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CorrectionSet {
    Start,
}
