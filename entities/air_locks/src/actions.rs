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

use api::{
    data::EntityDataResource,
    data_link::{DataLink, DataLinkType},
    entity_updates::EntityData,
    gridmap::{CellData, GridMapType},
    inventory::Inventory,
    tab_actions::QueuedTabActions,
};
use bevy::prelude::{Entity, EventWriter, Query, Res};

use super::air_lock_added::{
    AirLockLockClosed, AirLockLockOpen, AirLockUnlock, InputAirLockToggleOpen,
};

pub fn toggle_open_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    distance < 3.
}

pub fn lock_open_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    data_link_component: &DataLink,
) -> bool {
    distance < 30.
        && data_link_component
            .links
            .contains(&DataLinkType::RemoteLock)
}

pub fn unlock_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    data_link_component: &DataLink,
) -> bool {
    distance < 30.
        && data_link_component
            .links
            .contains(&DataLinkType::RemoteLock)
}

pub fn lock_closed_action(
    _self_tab_entity: Option<Entity>,
    _entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    data_link_component: &DataLink,
) -> bool {
    distance < 30.
        && data_link_component
            .links
            .contains(&DataLinkType::RemoteLock)
}
