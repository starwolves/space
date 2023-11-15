use bevy::prelude::Event;

use crate::cache::PhysicsCache;

#[derive(Event, Clone)]
pub struct CorrectionResults {
    pub data: PhysicsCache,
}
