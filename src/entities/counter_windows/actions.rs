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
use bevy::prelude::{Entity, EventWriter, Query, Res};

use crate::core::{
    data_link::data_link::{DataLink, DataLinkType},
    entity::entity_data::{EntityData, EntityDataResource},
    gridmap::gridmap::CellData,
    inventory::inventory::Inventory,
    networking::networking::GridMapType,
    tab_actions::tab_action::QueuedTabActions,
};

use super::counter_window_events::{
    CounterWindowLockClosed, CounterWindowLockOpen, CounterWindowUnlock,
    InputCounterWindowToggleOpen,
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
