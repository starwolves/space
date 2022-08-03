use bevy::{
    prelude::{Query, Res},
    time::Time,
};

use super::counter_window_events::CounterWindow;

pub fn counter_window_tick_timers(mut counter_windows: Query<&mut CounterWindow>, time: Res<Time>) {
    for mut counter_window_component in counter_windows.iter_mut() {
        match counter_window_component.denied_timer.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
        match counter_window_component.open_timer.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
        match counter_window_component.closed_timer.as_mut() {
            Some(x) => {
                x.tick(time.delta());
            }
            None => {}
        }
    }
}
