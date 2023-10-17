use actions::core::{ActionRequests, BuildingActions};
use bevy::prelude::{warn, Res, ResMut};
use entity::examine::ExamineEntityMessages;

/// Pawn examine action prerequisite check.

pub(crate) fn examine_prerequisite_check(mut building_action_data: ResMut<BuildingActions>) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "actions::pawn/examine" {
                action.approve();
            }
        }
    }
}
use entity::examine::InputExamineEntity;
use networking::server::HandleToEntity;

/// Examine.

pub(crate) fn examine(
    building_action_data: Res<BuildingActions>,
    mut examine_entity_messages: ResMut<ExamineEntityMessages>,
    handle_to_entity: Res<HandleToEntity>,
    action_requests: Res<ActionRequests>,
) {
    for building in building_action_data.list.iter() {
        let building_action_id;
        match action_requests.list.get(&building.incremented_i) {
            Some(action_request) => {
                building_action_id = action_request.get_id();
            }
            None => {
                continue;
            }
        }
        for action in building.actions.iter() {
            if action.is_approved()
                && action.data.id == "actions::pawn/examine"
                && action.data.id == building_action_id
            {
                match handle_to_entity.inv_map.get(&building.action_taker) {
                    Some(handle) => match building.target_entity_option {
                        Some(ex) => {
                            examine_entity_messages.messages.push(InputExamineEntity {
                                handle: *handle,
                                examine_entity: ex,
                                entity: building.action_taker,
                                ..Default::default()
                            });
                        }
                        None => {}
                    },
                    None => {
                        warn!("Couldnt find examiner in handletoentity.");
                    }
                }
            }
        }
    }
}
use actions::core::{Action, ActionData};
use bevy::prelude::Query;
use entity::examine::Examinable;

/// Build examine action.

pub(crate) fn build_actions(
    mut building_action_data: ResMut<BuildingActions>,
    examinable_items: Query<&Examinable>,
) {
    for building_action in building_action_data.list.iter_mut() {
        let mut new_vec = vec![ActionData {
            data: Action {
                id: "actions::pawn/examine".to_string(),
                text: "Examine".to_string(),
                tab_list_priority: u8::MAX,
            },
            approved: None,
        }];
        match building_action.target_entity_option {
            Some(examined_entity) => match examinable_items.get(examined_entity) {
                Ok(_) => {
                    building_action.actions.append(&mut new_vec);
                }
                Err(_rr) => {}
            },
            None => {
                building_action.actions.append(&mut new_vec);
            }
        }
    }
}
