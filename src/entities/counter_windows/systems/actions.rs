use bevy_app::EventWriter;
use bevy_ecs::{entity::Entity, system::Res};

use crate::{
    core::tab_actions::resources::QueuedTabActions,
    entities::counter_windows::events::{
        CounterWindowLockClosed, CounterWindowLockOpen, CounterWindowUnlock,
        InputCounterWindowToggleOpen,
    },
};

pub fn actions(
    queue: Res<QueuedTabActions>,

    mut counter_window_toggle_open_event: EventWriter<InputCounterWindowToggleOpen>,
    mut counter_window_lock_open_event: EventWriter<CounterWindowLockOpen>,
    mut counter_window_lock_closed_event: EventWriter<CounterWindowLockClosed>,
    mut counter_window_unlock_event: EventWriter<CounterWindowUnlock>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "actions::counter_windows/toggleopen" {
            if queued.target_entity_option.is_some() {
                counter_window_toggle_open_event.send(InputCounterWindowToggleOpen {
                    opener: queued.player_entity,
                    opened: queued.target_entity_option.unwrap(),
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::counter_windows/lockopen" {
            if queued.target_entity_option.is_some() {
                counter_window_lock_open_event.send(CounterWindowLockOpen {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::counter_windows/lockclosed" {
            if queued.target_entity_option.is_some() {
                counter_window_lock_closed_event.send(CounterWindowLockClosed {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::counter_windows/unlock" {
            if queued.target_entity_option.is_some() {
                counter_window_unlock_event.send(CounterWindowUnlock {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        }
    }
}
