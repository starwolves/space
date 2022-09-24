use bevy::prelude::ResMut;

use crate::data::{
    ActionIncremented, ActionRequest, ActionRequests, BuildingAction, BuildingActions,
    ListActionDataRequests,
};
use bevy::prelude::EventReader;
use networking::messages::InputAction;

/// Clears all actions for the next tick.
pub(crate) fn clear_action_building(
    mut action_data_requests: ResMut<ListActionDataRequests>,
    mut action_requests: ResMut<ActionRequests>,
    mut building_action: ResMut<BuildingActions>,
) {
    action_data_requests.list.clear();
    action_requests.list.clear();
    building_action.list.clear();
}

/// Initialize action (list) requests.
pub(crate) fn init_action_request_building(
    mut building_actions: ResMut<BuildingActions>,
    mut action_events: EventReader<InputAction>,
    mut action_data_i: ResMut<ActionIncremented>,
    mut actions_requests: ResMut<ActionRequests>,
) {
    for event in action_events.iter() {
        let mut examined_cell = None;

        match event.target_cell_option.clone() {
            Some(ya) => {
                examined_cell = Some((ya.1, ya.0));
            }
            None => {}
        }

        building_actions.list.push(BuildingAction {
            actions: vec![],
            incremented_i: action_data_i.get_i_it(),
            action_taker: event.action_taker,
            target_entity_option: event.target_entity_option,
            target_cell_option: examined_cell,
        });
        actions_requests.list.insert(
            action_data_i.get_i(),
            ActionRequest::from_id(event.fired_action_id.clone()),
        );
    }
}
