use api::actions::{Action, QueuedActions};
use bevy::prelude::ResMut;

use crate::data::{
    ActionDataRequests, ActionIncremented, ActionRequest, ActionRequests, BuildingActions,
    BuildingActionsInstance,
};
use bevy::prelude::{info, EventReader};
use networking::messages::InputAction;

pub fn get_action(id: &str) -> Option<Action> {
    let result;

    if id == "actions::pawn/examine" {
        result = Some(Action {
            id: id.to_string(),
            text: "Examine".to_string(),
            tab_list_priority: u8::MAX,
        });
    } else if id == "actions::inventory/pickup" {
        result = Some(Action {
            id: id.to_string(),
            text: "Pickup".to_string(),
            tab_list_priority: u8::MAX - 1,
        });
    } else {
        result = None;
    }

    result
}

pub fn clear_actions_queue(mut queue: ResMut<QueuedActions>) {
    queue.queue.clear();
}

pub fn clear_action_building(
    mut action_data_requests: ResMut<ActionDataRequests>,
    mut action_requests: ResMut<ActionRequests>,
    mut building_action: ResMut<BuildingActions>,
) {
    action_data_requests.list.clear();
    action_requests.list.clear();
    if building_action.list.len() > 0 {
        info!("Cleared action data!");
    }
    building_action.list.clear();
}

pub fn init_action_building(
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

        building_actions.list.push(BuildingActionsInstance {
            actions: vec![],
            incremented_i: action_data_i.get_i_it(),
            action_taker: event.action_taker,
            target_entity_option: event.target_entity_option,
            target_cell_option: examined_cell,
        });
        actions_requests.list.insert(
            action_data_i.get_i(),
            ActionRequest {
                id: event.fired_action_id.clone(),
            },
        );
    }
}
