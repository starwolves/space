use bevy::{
    ecs::{event::Event, schedule::SystemSet, system::Resource},
    log::warn,
};

#[derive(Debug, Event, Clone, Resource, Default)]
pub struct StartCorrection {
    /// The tick at which correction simulation should sync. This tick does not get returned and re-applied in correction results.
    pub start_tick: u64,
    /// The last tick that is calculated. This should always be equal to TickRateStamp.large of the loop it gets called in.
    pub last_tick: u64,
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CorrectionServerSet {
    TriggerSync,
    SyncClear,
}

pub const MAX_CACHE_TICKS_AMNT: u64 = 32;

#[derive(Resource, Default)]
pub struct IsCorrecting(pub bool);

#[derive(Resource)]
pub struct SynchronousCorrection(pub bool);

#[derive(Resource, Default)]
pub struct SynchronousCorrectionOnGoing(pub Vec<bool>);
impl SynchronousCorrectionOnGoing {
    pub fn receive_ready(&self) -> bool {
        for b in self.0.iter() {
            if *b {
                return true;
            }
        }
        return false;
    }
    pub fn send_ready(&self) -> bool {
        for b in self.0.iter() {
            if !*b {
                return true;
            }
        }
        return false;
    }
    pub fn step(&mut self) {
        for b in self.0.iter_mut() {
            if *b == true {
                warn!("SynchronousCorrectionOnGoing stepped twice.");
            }
            *b = true;
        }
    }
}
#[derive(Resource, Default)]
pub struct ObtainedSynchronousSyncData(pub bool);
