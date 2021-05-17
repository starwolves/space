use bevy::{core::{Time, Timer}, prelude::{Query, Res}};

use crate::space_core::components::{air_lock_denied_timer::AirLockDeniedTimer, air_lock_open_timer::AirLockOpenTimer};

pub fn tick_timers(
    time: Res<Time>, 
    mut query_timer: Query<&mut Timer>,
    mut query_air_lock_denied_timer: Query<&mut AirLockDeniedTimer>,
    mut query_air_lock_open_timer: Query<&mut AirLockOpenTimer>,
) {
    for mut timer in query_timer.iter_mut() {
        timer.tick(time.delta());
    }
    for mut timer in query_air_lock_denied_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
    for mut timer in query_air_lock_open_timer.iter_mut() {
        timer.timer.tick(time.delta());
    }
}
