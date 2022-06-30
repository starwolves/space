use bevy_core::Time;
use bevy_ecs::system::{Query, Res};

use crate::entities::air_locks::components::AirLock;

pub fn air_lock_tick_timers(time: Res<Time>, mut air_locks: Query<&mut AirLock>) {
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
