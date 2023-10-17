use actions::core::{Action, ActionData, ActionRequests, BuildingActions};
use bevy::prelude::{warn, EventWriter, Query, Res, ResMut, Transform};
use pawn::pawn::{DataLink, DataLinkType};
use resources::math::{cell_id_to_world, Vec3Int};

use crate::{
    airlock_events::{AirLockLockOpen, AirlockLockClosed, AirlockUnlock, InputAirlockToggleOpen},
    resources::Airlock,
};

/// Action prerequite check.

pub(crate) fn toggle_open_action_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    transforms: Query<&Transform>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "actions::airlocks/toggleopen" {
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

/// Manage air lock actions.

pub(crate) fn airlock_actions(
    building_action: Res<BuildingActions>,
    mut airlock_lock_open_event: EventWriter<AirLockLockOpen>,
    mut airlock_lock_closed_event: EventWriter<AirlockLockClosed>,
    mut airlock_unlock_event: EventWriter<AirlockUnlock>,
    mut toggle_open_events: EventWriter<InputAirlockToggleOpen>,
    handle_to_entity: Res<HandleToEntity>,
    action_requests: Res<ActionRequests>,
) {
    for building in building_action.list.iter() {
        let building_action_id;
        match action_requests.list.get(&building.incremented_i) {
            Some(action_request) => {
                building_action_id = action_request.get_id();
            }
            None => {
                continue;
            }
        }
        for action_data in building.actions.iter() {
            if action_data.is_approved()
                && action_data.data.id == "actions::airlocks/lockopen"
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
                airlock_lock_open_event.send(AirLockLockOpen {
                    handle_option,
                    locker: building.action_taker,
                    locked: building.target_entity_option.unwrap(),
                });
            }
            if action_data.is_approved()
                && action_data.data.id == "actions::airlocks/lockclosed"
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
                airlock_lock_closed_event.send(AirlockLockClosed {
                    handle_option,
                    locker: building.action_taker,
                    locked: building.target_entity_option.unwrap(),
                });
            }
            if action_data.is_approved()
                && action_data.data.id == "actions::airlocks/unlock"
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
                airlock_unlock_event.send(AirlockUnlock {
                    handle_option,
                    locker: building.action_taker,
                    locked: building.target_entity_option.unwrap(),
                });
            }
            if action_data.is_approved()
                && action_data.data.id == "actions::airlocks/toggleopen"
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
                toggle_open_events.send(InputAirlockToggleOpen {
                    handle_option,
                    opener: building.action_taker,
                    opened: building.target_entity_option.unwrap(),
                });
            }
        }
    }
}

/// Prerequisite check of locking an airlock.

pub(crate) fn lock_action_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    examiner: Query<(&Transform, &DataLink)>,
    transforms: Query<&Transform>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "actions::airlocks/lockopen"
                || action.data.id == "actions::airlocks/lockclosed"
                || action.data.id == "actions::airlocks/unlock"
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
                    Some(target_entity_bits) => match transforms.get(target_entity_bits) {
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

/// Build air lock actions.

pub(crate) fn build_actions(
    mut building_action_data: ResMut<BuildingActions>,
    airlocks: Query<&Airlock>,
) {
    for building_action in building_action_data.list.iter_mut() {
        match building_action.target_entity_option {
            Some(examined_entity) => match airlocks.get(examined_entity) {
                Ok(_) => {
                    let mut new_vec = vec![
                        ActionData {
                            data: Action {
                                id: "actions::airlocks/toggleopen".to_string(),
                                text: "Toggle Open".to_string(),
                                tab_list_priority: 100,
                            },
                            approved: None,
                        },
                        ActionData {
                            data: Action {
                                id: "actions::airlocks/lockopen".to_string(),
                                text: "Lock Open".to_string(),
                                tab_list_priority: 99,
                            },
                            approved: None,
                        },
                        ActionData {
                            data: Action {
                                id: "actions::airlocks/lockclosed".to_string(),
                                text: "Lock Closed".to_string(),
                                tab_list_priority: 98,
                            },
                            approved: None,
                        },
                        ActionData {
                            data: Action {
                                id: "actions::airlocks/unlock".to_string(),
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
