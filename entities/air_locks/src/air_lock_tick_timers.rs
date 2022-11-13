use bevy::{
    prelude::{Query, Res},
    time::Time,
};

use super::resources::AirLock;

/// Air lock tick timers.
#[cfg(feature = "server")]
pub(crate) fn air_lock_tick_timers(time: Res<Time>, mut air_locks: Query<&mut AirLock>) {
    for mut air_lock_component in air_locks.iter_mut() {
        match air_lock_component.denied_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
        match air_lock_component.open_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
        match air_lock_component.closed_timer_option.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
    }
}
