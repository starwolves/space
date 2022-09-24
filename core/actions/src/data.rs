use std::collections::HashMap;

use api::{
    data::{HandleToEntity, Vec3Int},
    examinable::Examinable,
    gridmap::GridMapType,
    network::{NetAction, ReliableServerMessage},
};
use bevy::prelude::{warn, Component, Entity, EventReader, EventWriter, Query, Res, ResMut, With};
use networking::{
    messages::{InputListActionsEntity, InputListActionsMap},
    plugin::NetActionData,
};

/// A list of actions being built.
#[derive(Default)]
pub struct BuildingActions {
    pub list: Vec<BuildingAction>,
}
/// Requests to execute actions which go through prerequisite checking.
#[derive(Default)]
pub struct ActionRequests {
    pub list: HashMap<u64, ActionRequest>,
}

/// A request to execute a request.
pub struct ActionRequest {
    /// Action identifier.
    id: String,
}

impl ActionRequest {
    /// Get identifier.
    pub fn get_id(&self) -> &str {
        &self.id
    }
    /// Create from identifier.
    pub fn from_id(new_id: String) -> Self {
        Self { id: new_id }
    }
    pub fn set_id(&mut self, new_id: String) {
        self.id = new_id;
    }
}

/// A request to build a list of available actions.
pub struct BuildingAction {
    /// Available to-be-approved actions
    pub actions: Vec<ActionData>,
    /// Build action request identifier.
    pub incremented_i: u64,
    /// The entity which we request action data for.
    pub action_taker: Entity,
    /// The entity targetted in the action.
    pub target_entity_option: Option<Entity>,
    /// The ship cell targetted in the action.
    pub target_cell_option: Option<(Vec3Int, GridMapType)>,
}

/// Data related to an individual action.
pub struct ActionData {
    /// The action.
    pub data: Action,
    /// Whether the action is approved or not by a prerequisite checker.
    pub approved: Option<bool>,
}

impl ActionData {
    /// Approve action, typically performed by prerequisite checks.
    pub fn approve(&mut self) {
        match self.approved {
            Some(_) => {}
            None => {
                self.approved = Some(true);
            }
        }
    }
    /// Do not approve action, typically performed by prerequisite checks.
    pub fn do_not_approve(&mut self) {
        self.approved = Some(false);
    }
    /// Check if action is approved.
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

/// Send lists of approved actions back to player.
pub(crate) fn list_action_data_finalizer(
    building_actions: Res<BuildingActions>,
    handle_to_entity: Res<HandleToEntity>,
    mut net: EventWriter<NetActionData>,
    action_data_requests: Res<ListActionDataRequests>,
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
            if action.is_approved() {
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
                            &action_data_request.get_id(),
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

/// Append actions found in [Actions] component of entities to their list.
pub(crate) fn list_action_data_from_actions_component(
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

/// A resource storing the current iterated identifiers of action (build) requests.
#[derive(Default)]
pub(crate) struct ActionIncremented {
    i: u64,
}

impl ActionIncremented {
    /// Get i with iteration.
    pub fn get_i_it(&mut self) -> u64 {
        let i = self.i.clone();
        self.i += 1;
        i
    }
    /// Get i without iterating.
    pub fn get_i(&self) -> u64 {
        if self.i > 0 {
            return self.i - 1;
        }
        self.i
    }
}

/// Request list of available actions for entity with prerequisite checking.
#[derive(Default)]
pub struct ListActionDataRequests {
    pub list: HashMap<u64, ActionRequest>,
}

/// Initialize listing action requests from input events.
pub(crate) fn init_action_data_listing(
    mut entity_events: EventReader<InputListActionsEntity>,
    mut map_events: EventReader<InputListActionsMap>,
    mut building_action: ResMut<BuildingActions>,
    mut action_data_i: ResMut<ActionIncremented>,
    mut action_data_requests: ResMut<ListActionDataRequests>,
) {
    for event in entity_events.iter() {
        building_action.list.push(BuildingAction {
            incremented_i: action_data_i.get_i_it(),
            actions: vec![],
            action_taker: event.player_entity,
            target_entity_option: Some(event.examine_entity_bits),
            target_cell_option: None,
        });
        action_data_requests.list.insert(
            action_data_i.get_i(),
            ActionRequest::from_id("".to_string()),
        );
    }
    for event in map_events.iter() {
        building_action.list.push(BuildingAction {
            incremented_i: action_data_i.get_i_it(),
            actions: vec![],
            action_taker: event.player_entity,
            target_entity_option: None,
            target_cell_option: Some((event.gridmap_cell_id, event.gridmap_type.clone())),
        });
        action_data_requests.list.insert(
            action_data_i.get_i(),
            ActionRequest::from_id("".to_string()),
        );
    }
}

/// An individual action.
#[derive(Clone)]
pub struct Action {
    pub id: String,
    pub text: String,
    pub tab_list_priority: u8,
}

/// A list of actions.
#[derive(Component, Default)]
pub struct Actions {
    pub actions: Vec<Action>,
}

impl Action {
    /// Convert action into a new struct suitable to be sent over the net.
    pub fn into_net(
        &self,
        item_name: &str,
        examined_entity_option: Option<Entity>,
        examined_cell_option: Option<(GridMapType, i16, i16, i16)>,
        examiner_entity: Entity,
    ) -> NetAction {
        let mut new_entity_option = None;
        match examined_entity_option {
            Some(b) => new_entity_option = Some(b.to_bits()),
            None => {}
        }

        NetAction {
            id: self.id.clone(),
            text: self.text.clone(),
            tab_list_priority: self.tab_list_priority,
            entity_option: new_entity_option,
            cell_option: examined_cell_option,
            item_name: item_name.to_string(),
            belonging_entity: Some(examiner_entity.to_bits()),
        }
    }
}
