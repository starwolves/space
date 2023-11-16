use bevy::ecs::{event::Event, schedule::SystemSet, system::Resource};

#[derive(Event, Clone, Resource, Default)]
pub struct StartCorrection {
    pub start_tick: u64,
    /// The last tick that is calculated.
    pub last_tick: u64,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum CorrectionSet {
    SyncData,
    Start,
}

#[derive(Resource, Default)]
pub struct SyncWorld {
    pub rebuild: bool,
    pub second_tick: bool,
    pub sync_to_tick: u64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CorrectionServerSet {
    TriggerSync,
    SyncClear,
}
