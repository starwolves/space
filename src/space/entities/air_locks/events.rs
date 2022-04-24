use bevy_ecs::entity::Entity;

use crate::space::core::{
    entity::components::EntityGroup, networking::resources::ReliableServerMessage,
};

pub struct AirLockCollision {
    pub collider1_entity: Entity,
    pub collider2_entity: Entity,

    pub collider1_group: EntityGroup,
    pub collider2_group: EntityGroup,

    pub started: bool,
}

pub struct InputAirLockToggleOpen {
    pub handle_option: Option<u32>,

    pub opener: Entity,
    pub opened: u64,
}

pub struct AirLockLockOpen {
    pub handle_option: Option<u32>,

    pub locked: Entity,
    pub locker: Entity,
}

pub struct AirLockLockClosed {
    pub handle_option: Option<u32>,

    pub locked: Entity,
    pub locker: Entity,
}

pub struct NetAirLock {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct AirLockUnlock {
    pub handle_option: Option<u32>,
    pub locked: Entity,
    pub locker: Entity,
}

use bevy_app::EventWriter;
use bevy_ecs::system::Res;

use crate::space::core::tab_actions::resources::QueuedTabActions;

pub fn air_locks_actions(
    queue: Res<QueuedTabActions>,
    mut air_lock_toggle_open_event: EventWriter<InputAirLockToggleOpen>,
    mut air_lock_lock_open_event: EventWriter<AirLockLockOpen>,
    mut air_lock_lock_closed_event: EventWriter<AirLockLockClosed>,
    mut air_lock_unlock_event: EventWriter<AirLockUnlock>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "actions::air_locks/toggleopen" {
            if queued.target_entity_option.is_some() {
                air_lock_toggle_open_event.send(InputAirLockToggleOpen {
                    opener: queued.player_entity,
                    opened: queued.target_entity_option.unwrap(),
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::air_locks/lockopen" {
            if queued.target_entity_option.is_some() {
                air_lock_lock_open_event.send(AirLockLockOpen {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::air_locks/lockclosed" {
            if queued.target_entity_option.is_some() {
                air_lock_lock_closed_event.send(AirLockLockClosed {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        } else if queued.tab_id == "actions::air_locks/unlock" {
            if queued.target_entity_option.is_some() {
                air_lock_unlock_event.send(AirLockUnlock {
                    locked: Entity::from_bits(queued.target_entity_option.unwrap()),
                    locker: queued.player_entity,
                    handle_option: queued.handle_option,
                });
            }
        }
    }
}
