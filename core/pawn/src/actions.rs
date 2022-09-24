use actions::data::{ActionRequests, BuildingActions};
use api::{
    data::HandleToEntity,
    examinable::InputExamineEntity,
    gridmap::{ExamineMapMessage, GridmapExamineMessages},
};
use bevy::prelude::{warn, Res, ResMut};
use networking::messages::ExamineEntityMessages;

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

/// Examine pawn.
pub(crate) fn examine(
    building_action_data: Res<BuildingActions>,
    mut examine_entity_messages: ResMut<ExamineEntityMessages>,
    mut examine_map_messages: ResMut<GridmapExamineMessages>,
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
                        None => {
                            let c = building.target_cell_option.clone().unwrap();

                            examine_map_messages.messages.push(ExamineMapMessage {
                                handle: *handle,
                                entity: building.action_taker,
                                gridmap_type: c.1,
                                gridmap_cell_id: c.0,
                                ..Default::default()
                            });
                        }
                    },
                    None => {
                        warn!("Couldnt find examiner in handletoentity.");
                    }
                }
            }
        }
    }
}
