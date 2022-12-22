use bevy::{
    prelude::{Query, Res},
    time::Time,
};

use super::resources::Airlock;

/// Air lock tick timers.
#[cfg(feature = "server")]
pub(crate) fn airlock_tick_timers(time: Res<Time>, mut airlocks: Query<&mut Airlock>) {
    for mut airlock_component in airlocks.iter_mut() {
        match airlock_component.denied_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
        match airlock_component.open_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
        match airlock_component.closed_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
    }
}
