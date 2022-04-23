use bevy_app::EventWriter;
use bevy_ecs::{entity::Entity, system::Res};

use crate::space::core::tab_actions::resources::QueuedTabActions;

use super::events::{CounterWindowLockClosed, CounterWindowLockOpen, InputCounterWindowToggleOpen};

pub fn counter_windows_actions(
    queue: Res<QueuedTabActions>,

    mut counter_window_toggle_open_event: EventWriter<InputCounterWindowToggleOpen>,
    mut counter_window_lock_open_event: EventWriter<CounterWindowLockOpen>,
    mut counter_window_lock_closed_event: EventWriter<CounterWindowLockClosed>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "counterwindowtoggleopen" {
            if queued.target_entity_option.is_some() {
                counter_window_toggle_open_event.send(InputCounterWindowToggleOpen {
                    opener: Entity::from_bits(queued.belonging_entity),
                    opened: queued.target_entity_option.unwrap(),
                });
            }
        } else if queued.tab_id == "counterwindowlockopen" {
            if queued.target_entity_option.is_some() {
                counter_window_lock_open_event.send(CounterWindowLockOpen {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: Entity::from_bits(queued.belonging_entity),
                });
            }
        } else if queued.tab_id == "counterwindowlockclosed" {
            if queued.target_entity_option.is_some() {
                counter_window_lock_closed_event.send(CounterWindowLockClosed {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: Entity::from_bits(queued.belonging_entity),
                });
            }
        }
    }
}
