use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::SystemSet;

use crate::space::PostUpdateLabels;

use self::{
    entity_update::air_lock_update,
    events::{AirLockCollision, AirLockLockClosed, AirLockLockOpen, InputAirLockToggleOpen},
    systems::{air_lock_added, air_lock_default_map_added, air_lock_events, air_lock_tick_timers},
};

pub mod components;
pub mod entity_update;
pub mod events;
pub mod functions;
pub mod spawn;
pub mod systems;

pub struct AirLocksPlugin;

impl Plugin for AirLocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AirLockCollision>()
            .add_event::<InputAirLockToggleOpen>()
            .add_event::<AirLockLockOpen>()
            .add_system(air_lock_added)
            .add_system(air_lock_tick_timers)
            .add_system(air_lock_default_map_added)
            .add_event::<AirLockLockClosed>()
            .add_system(air_lock_events)
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(air_lock_update),
            );
    }
}
