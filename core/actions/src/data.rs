use std::collections::HashMap;

use api::{
    actions::{Action, Actions},
    data::{HandleToEntity, Vec3Int},
    examinable::Examinable,
    gridmap::GridMapType,
    network::ReliableServerMessage,
};
use bevy::prelude::{info, warn, Entity, EventReader, EventWriter, Query, Res, ResMut, With};
use networking::{
    messages::{InputActionDataEntity, InputActionDataMap},
    plugin::NetActionData,
};

#[derive(Default)]
pub struct BuildingActions {
    pub list: Vec<BuildingActionsInstance>,
}
#[derive(Default)]
pub struct ActionRequests {
    pub list: HashMap<u64, ActionRequest>,
}

pub struct ActionRequest {
    pub id: String,
}

pub struct BuildingActionsInstance {
    pub actions: Vec<ActionData>,
    pub incremented_i: u64,
    pub action_taker: Entity,
    pub target_entity_option: Option<Entity>,
    pub target_cell_option: Option<(Vec3Int, GridMapType)>,
}

pub struct ActionData {
    pub data: Action,
    pub approved: Option<bool>,
}

impl ActionData {
    pub fn approve(&mut self) {
        match self.approved {
            Some(_) => {}
            None => {
                self.approved = Some(true);
            }
        }
    }
    pub fn do_not_approve(&mut self) {
        self.approved = Some(false);
    }
    pub fn is_approved(&self) -> bool {
        match self.approved {
            Some(_) => {
                return true;
            }
            None => {}
        }
        return false;
    }
}

pub fn action_data_finalizer(
    building_actions: Res<BuildingActions>,
    handle_to_entity: Res<HandleToEntity>,
    mut net: EventWriter<NetActionData>,
    action_data_requests: Res<ActionDataRequests>,
) {
    for action_data in building_actions.list.iter() {
        let action_data_request;
        match action_data_requests.list.get(&action_data.incremented_i) {
            Some(d) => {
                action_data_request = d;
            }
            None => {
                continue;
            }
        }
        let mut net_action_datas = vec![];
        let mut handle = None;

        for action in action_data.actions.iter() {
            if action.approved.is_some() && action.approved.unwrap() {
                match handle_to_entity.inv_map.get(&action_data.action_taker) {
                    Some(h) => {
                        handle = Some(*h);
                        let mut cell_option = None;

                        match action_data.target_cell_option.clone() {
                            Some(c) => {
                                cell_option = Some((c.1, c.0.x, c.0.y, c.0.z));
                            }
                            None => {}
                        }

                        net_action_datas.push(action.data.into_net(
                            &action_data_request.header_name,
                            action_data.target_entity_option,
                            cell_option,
                            action_data.action_taker,
                        ));
                    }
                    None => {
                        warn!("No entity handle for tab data request!");
                        continue;
                    }
                }
            }
        }

        match handle {
            Some(h) => {
                net.send(NetActionData {
                    handle: h,
                    message: ReliableServerMessage::TabData(net_action_datas),
                });
            }
            None => {}
        }
    }
}

pub fn action_data_build_interacted_entity(
    examinable_query: Query<&Actions, With<Examinable>>,
    mut building_actions: ResMut<BuildingActions>,
) {
    for building_action in building_actions.list.iter_mut() {
        match examinable_query.get(building_action.action_taker) {
            Ok(actions_component) => {
                for action in actions_component.actions.iter() {
                    building_action.actions.push(ActionData {
                        data: action.clone(),
                        approved: None,
                    });
                }
            }
            Err(_rr) => {}
        }
    }
}
#[derive(Default)]
pub struct ActionIncremented {
    pub i: u64,
}

impl ActionIncremented {
    pub fn get_i_it(&mut self) -> u64 {
        let i = self.i.clone();
        self.i += 1;
        i
    }
    pub fn get_i(&self) -> u64 {
        if self.i > 0 {
            return self.i - 1;
        }
        self.i
    }
}
#[derive(Default)]
pub struct ActionDataRequests {
    pub list: HashMap<u64, ActionDataRequest>,
}

pub struct ActionDataRequest {
    pub header_name: String,
}

pub fn init_action_data_building(
    mut entity_events: EventReader<InputActionDataEntity>,
    mut map_events: EventReader<InputActionDataMap>,
    mut building_action: ResMut<BuildingActions>,
    mut action_data_i: ResMut<ActionIncremented>,
    mut action_data_requests: ResMut<ActionDataRequests>,
) {
    for event in entity_events.iter() {
        building_action.list.push(BuildingActionsInstance {
            incremented_i: action_data_i.get_i_it(),
            actions: vec![],
            action_taker: event.player_entity,
            target_entity_option: Some(event.examine_entity_bits),
            target_cell_option: None,
        });
        action_data_requests.list.insert(
            action_data_i.get_i(),
            ActionDataRequest {
                header_name: "".to_string(),
            },
        );
    }
    for event in map_events.iter() {
        info!("Tab data build requested.");
        building_action.list.push(BuildingActionsInstance {
            incremented_i: action_data_i.get_i_it(),
            actions: vec![],
            action_taker: event.player_entity,
            target_entity_option: None,
            target_cell_option: Some((event.gridmap_cell_id, event.gridmap_type.clone())),
        });
        action_data_requests.list.insert(
            action_data_i.get_i(),
            ActionDataRequest {
                header_name: "".to_string(),
            },
        );
    }
}
