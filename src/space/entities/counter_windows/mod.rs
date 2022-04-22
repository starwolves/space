use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::SystemSet;

use crate::space::PostUpdateLabels;

use self::{
    entity_update::counter_window_update,
    events::{
        CounterWindowLockClosed, CounterWindowLockOpen, CounterWindowSensorCollision,
        InputCounterWindowToggleOpen,
    },
    systems::{
        counter_window_added, counter_window_default_map_added, counter_window_events,
        counter_window_tick_timers,
    },
};

pub mod components;
pub mod entity_update;
pub mod events;
pub mod functions;
pub mod spawn;
pub mod systems;

pub struct CounterWindowsPlugin;

impl Plugin for CounterWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CounterWindowSensorCollision>()
            .add_system(counter_window_events)
            .add_system(counter_window_tick_timers)
            .add_system(counter_window_added)
            .add_system(counter_window_default_map_added)
            .add_event::<InputCounterWindowToggleOpen>()
            .add_event::<CounterWindowLockOpen>()
            .add_event::<CounterWindowLockClosed>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .label(PostUpdateLabels::EntityUpdate)
                    .with_system(counter_window_update),
            );
    }
}
