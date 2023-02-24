use actions::core::{Action, ActionData, ActionRequests, BuildingActions};
use bevy::prelude::{warn, EventWriter, Query, Res, ResMut, Transform};
use pawn::pawn::{DataLink, DataLinkType};
use resources::math::{cell_id_to_world, Vec3Int};

use crate::counter_window_events::CounterWindow;

use super::counter_window_events::{
    CounterWindowLockClosed, CounterWindowLockOpen, CounterWindowUnlock,
    InputCounterWindowToggleOpen,
};

pub(crate) fn toggle_open_action_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    transforms: Query<&Transform>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "actions::counter_windows/toggleopen" {
                let examiner_transform;

                match transforms.get(building.action_taker) {
                    Ok(t) => {
                        examiner_transform = t;
                    }
                    Err(_rr) => {
                        warn!("Couldnt find transform of examining entity!");
                        continue;
                    }
                }

                let start_pos;
                let end_pos = examiner_transform.translation;

                match building.target_entity_option.clone() {
                    Some(_target_entity_bits) => match transforms.get(building.action_taker) {
                        Ok(rigid_body_position) => {
                            start_pos = rigid_body_position.translation;
                        }
                        Err(_) => {
                            continue;
                        }
                    },
                    None => {
                        let cell_data;
                        match building.target_cell_option.as_ref() {
                            Some(v) => {
                                cell_data = v;
                            }
                            None => {
                                continue;
                            }
                        }
                        start_pos = cell_id_to_world(Vec3Int {
                            x: cell_data.id.x,
                            y: cell_data.id.y,
                            z: cell_data.id.z,
                        });
                    }
                }

                let distance = start_pos.distance(end_pos);

                match distance < 3. {
                    true => {
                        action.approve();
                    }
                    false => {
                        action.do_not_approve();
                    }
                }
            }
        }
    }
}
use networking::server::HandleToEntity;

pub(crate) fn counter_window_actions(
    building_action_data: Res<BuildingActions>,
    mut counter_window_lock_open_event: EventWriter<CounterWindowLockOpen>,
    mut counter_window_lock_closed_event: EventWriter<CounterWindowLockClosed>,
    mut counter_window_unlock_event: EventWriter<CounterWindowUnlock>,
    mut counter_window_toggle_open_event: EventWriter<InputCounterWindowToggleOpen>,
    handle_to_entity: Res<HandleToEntity>,
    action_requests: Res<ActionRequests>,
) {
    for building in building_action_data.list.iter() {
        let building_action_id;
        match action_requests.list.get(&building.incremented_i) {
            Some(action_request) => {
                building_action_id = action_request.get_id().clone();
            }
            None => {
                continue;
            }
        }
        for action_data in building.actions.iter() {
            if action_data.is_approved()
                && action_data.data.id == "actions::counter_windows/lockopen"
                && action_data.data.id == building_action_id
            {
                let handle_option;
                match handle_to_entity.inv_map.get(&building.action_taker) {
                    Some(h) => {
                        handle_option = Some(*h);
                    }
                    None => {
                        handle_option = None;
                    }
                }
                counter_window_lock_open_event.send(CounterWindowLockOpen {
                    handle_option,
                    locker: building.action_taker,
                    locked: building.target_entity_option.unwrap(),
                });
            } else if action_data.is_approved()
                && action_data.data.id == "actions::counter_windows/lockclosed"
                && action_data.data.id == building_action_id
            {
                let handle_option;
                match handle_to_entity.inv_map.get(&building.action_taker) {
                    Some(h) => {
                        handle_option = Some(*h);
                    }
                    None => {
                        handle_option = None;
                    }
                }
                counter_window_lock_closed_event.send(CounterWindowLockClosed {
                    handle_option,
                    locker: building.action_taker,
                    locked: building.target_entity_option.unwrap(),
                });
            } else if action_data.is_approved()
                && action_data.data.id == "actions::counter_windows/unlock"
                && action_data.data.id == building_action_id
            {
                let handle_option;
                match handle_to_entity.inv_map.get(&building.action_taker) {
                    Some(h) => {
                        handle_option = Some(*h);
                    }
                    None => {
                        handle_option = None;
                    }
                }
                counter_window_unlock_event.send(CounterWindowUnlock {
                    handle_option,
                    locker: building.action_taker,
                    locked: building.target_entity_option.unwrap(),
                });
            } else if action_data.is_approved()
                && action_data.data.id == "actions::counter_windows/toggleopen"
                && action_data.data.id == building_action_id
            {
                let handle_option;
                match handle_to_entity.inv_map.get(&building.action_taker) {
                    Some(h) => {
                        handle_option = Some(*h);
                    }
                    None => {
                        handle_option = None;
                    }
                }
                counter_window_toggle_open_event.send(InputCounterWindowToggleOpen {
                    handle_option,
                    opener: building.action_taker,
                    opened: building.target_entity_option.unwrap(),
                });
            }
        }
    }
}

pub(crate) fn lock_open_action_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    examiner: Query<(&Transform, &DataLink)>,
    transforms: Query<&Transform>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "actions::counter_windows/lockopen"
                || action.data.id == "actions::counter_windows/lockclosed"
                || action.data.id == "actions::counter_windows/unlock"
            {
                let examiner_transform;
                let examiner_data_link;

                match examiner.get(building.action_taker) {
                    Ok((t, d)) => {
                        examiner_transform = t;
                        examiner_data_link = d;
                    }
                    Err(_rr) => {
                        warn!("Couldnt find transform of examining entity!");
                        continue;
                    }
                }

                let start_pos;
                let end_pos = examiner_transform.translation;

                match building.target_entity_option.clone() {
                    Some(_target_entity_bits) => match transforms.get(building.action_taker) {
                        Ok(rigid_body_position) => {
                            start_pos = rigid_body_position.translation;
                        }
                        Err(_) => {
                            continue;
                        }
                    },
                    None => {
                        let cell_data;
                        match building.target_cell_option.as_ref() {
                            Some(v) => {
                                cell_data = v;
                            }
                            None => {
                                continue;
                            }
                        }
                        start_pos = cell_id_to_world(Vec3Int {
                            x: cell_data.id.x,
                            y: cell_data.id.y,
                            z: cell_data.id.z,
                        });
                    }
                }

                let distance = start_pos.distance(end_pos);

                match distance < 30. && examiner_data_link.links.contains(&DataLinkType::RemoteLock)
                {
                    true => {
                        action.approve();
                    }
                    false => {
                        action.do_not_approve();
                    }
                }
            }
        }
    }
}

pub(crate) fn build_actions(
    mut building_action_data: ResMut<BuildingActions>,
    counter_windows: Query<&CounterWindow>,
) {
    for building_action in building_action_data.list.iter_mut() {
        match building_action.target_entity_option {
            Some(examined_entity) => match counter_windows.get(examined_entity) {
                Ok(_) => {
                    let mut new_vec = vec![
                        ActionData {
                            data: Action {
                                id: "actions::counter_windows/toggleopen".to_string(),
                                text: "Toggle Open".to_string(),
                                tab_list_priority: 100,
                            },
                            approved: None,
                        },
                        ActionData {
                            data: Action {
                                id: "actions::counter_windows/lockopen".to_string(),
                                text: "Lock Open".to_string(),
                                tab_list_priority: 99,
                            },
                            approved: None,
                        },
                        ActionData {
                            data: Action {
                                id: "actions::counter_windows/lockclosed".to_string(),
                                text: "Lock Closed".to_string(),
                                tab_list_priority: 98,
                            },
                            approved: None,
                        },
                        ActionData {
                            data: Action {
                                id: "actions::counter_windows/unlock".to_string(),
                                text: "Unlock".to_string(),
                                tab_list_priority: 97,
                            },
                            approved: None,
                        },
                    ];

                    building_action.actions.append(&mut new_vec);
                }
                Err(_rr) => {}
            },
            None => {}
        }
    }
}
