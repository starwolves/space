use bevy_app::EventWriter;
use bevy_ecs::{entity::Entity, system::Res};

use crate::space::core::tab_actions::resources::QueuedTabActions;

use super::events::{AirLockLockClosed, AirLockLockOpen, InputAirLockToggleOpen};

pub fn air_locks_actions(
    queue: Res<QueuedTabActions>,
    mut air_lock_toggle_open_event: EventWriter<InputAirLockToggleOpen>,
    mut air_lock_lock_open_event: EventWriter<AirLockLockOpen>,
    mut air_lock_lock_closed_event: EventWriter<AirLockLockClosed>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "airlocktoggleopen" {
            if queued.target_entity_option.is_some() {
                air_lock_toggle_open_event.send(InputAirLockToggleOpen {
                    opener: Entity::from_bits(queued.belonging_entity),
                    opened: queued.target_entity_option.unwrap(),
                });
            }
        } else if queued.tab_id == "airlocklockopen" {
            if queued.target_entity_option.is_some() {
                air_lock_lock_open_event.send(AirLockLockOpen {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: Entity::from_bits(queued.belonging_entity),
                });
            }
        } else if queued.tab_id == "airlocklockclosed" {
            if queued.target_entity_option.is_some() {
                air_lock_lock_closed_event.send(AirLockLockClosed {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: Entity::from_bits(queued.belonging_entity),
                });
            }
        }
    }
}
