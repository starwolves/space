use bevy::ecs::{event::Event, schedule::SystemSet, system::Resource};

#[derive(Debug, Event, Clone, Resource, Default)]
pub struct StartCorrection {
    pub start_tick: u64,
    /// The last tick that is calculated.
    pub last_tick: u64,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum CorrectionSet {
    Start,
}

#[derive(Resource, Default, Debug)]
pub struct SyncWorld {
    pub rebuild: bool,
    pub second_tick: bool,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CorrectionServerSet {
    TriggerSync,
    SyncClear,
}

pub const MAX_CACHE_TICKS_AMNT: u64 = 256;
