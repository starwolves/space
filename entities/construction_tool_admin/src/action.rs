use actions::core::{Action, ActionData, ActionRequests, BuildingActions};
use bevy::prelude::{warn, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform};
use gridmap::grid::{cell_id_to_world, GridmapMain};
use inventory::inventory::Inventory;
use pawn::pawn::REACH_DISTANCE;

use crate::construction_tool::{ConstructionTool, InputConstructionOptions, InputDeconstruct};
use networking::server::HandleToEntity;

use super::construction_tool::InputConstruct;

/// Manage construction actions.

pub(crate) fn construction_tool_actions(
    building_action_data: Res<BuildingActions>,
    handle_to_entity: Res<HandleToEntity>,
    mut event_construct: EventWriter<InputConstruct>,
    mut event_deconstruct: EventWriter<InputDeconstruct>,
    mut event_construction_options: EventWriter<InputConstructionOptions>,
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
                && action_data.data.id == "action::construction_tool_admin/construct"
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
                event_construct.send(InputConstruct {
                    handle_option,
                    target_cell: building.target_cell_option.clone().unwrap(),
                    belonging_entity: building.action_taker,
                });
            } else if action_data.is_approved()
                && action_data.data.id == "action::construction_tool_admin/deconstruct"
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
                event_deconstruct.send(InputDeconstruct {
                    handle_option,
                    target_cell_option: building.target_cell_option.clone(),
                    belonging_entity: building.action_taker,
                    target_entity_option: building.target_entity_option,
                });
            } else if action_data.is_approved()
                && action_data.data.id == "action::construction_tool_admin/constructionoptions"
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
                event_construction_options.send(InputConstructionOptions {
                    handle_option,
                    belonging_entity: building.action_taker,
                });
            }
        }
    }
}

pub fn construct_action_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    gridmap_main: Res<GridmapMain>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "action::construction_tool_admin/construct" {
                let cell_id;

                match building.target_cell_option.clone() {
                    Some(c) => {
                        cell_id = c.0;
                    }
                    None => {
                        warn!("couldnt find examined cell.");
                        continue;
                    }
                }

                let cell_option = gridmap_main.entity_data.get(&cell_id);

                match building.target_cell_option.is_some() && cell_option.is_none() {
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

pub(crate) fn deconstruct_action_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    gridmap_main: Res<GridmapMain>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "action::construction_tool_admin/deconstruct" {
                let cell_id;

                match building.target_cell_option.clone() {
                    Some(c) => {
                        cell_id = c.0;
                    }
                    None => {
                        warn!("got entity with cell action.");
                        continue;
                    }
                }

                let cell_option = gridmap_main.grid_data.get(&cell_id);

                match building.target_cell_option.is_some() && cell_option.is_some() {
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

pub(crate) fn construction_tool_search_distance_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    transforms: Query<&Transform>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == "action::construction_tool_admin/deconstruct"
                || action.data.id == "action::construction_tool_admin/construct"
            {
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

                let cell_id;

                match building.target_cell_option.clone() {
                    Some(c) => {
                        cell_id = c.0;
                        start_pos = cell_id_to_world(cell_id);
                    }
                    None => {
                        warn!("got entity with cell action.");
                        continue;
                    }
                }

                let distance = start_pos.distance(end_pos);

                if distance < REACH_DISTANCE {
                    action.approve();
                } else {
                    action.do_not_approve();
                }
            }
        }
    }
}

pub(crate) fn construction_tool_is_holding_item_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    inventory_holders: Query<&Inventory>,
    construction_tools: Query<&ConstructionTool>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id.contains("action::construction_tool_admin") {
                let mut passed = false;

                match inventory_holders.get(building.action_taker) {
                    Ok(i) => {
                        let active_slot = i.get_slot(&i.active_slot);
                        match active_slot.slot_item {
                            Some(e) => {
                                passed = construction_tools.get(e).is_ok();
                            }
                            None => {}
                        }
                    }
                    Err(_) => {
                        warn!("checker wasnt inventory holder.");
                        continue;
                    }
                }

                if passed {
                    action.approve();
                } else {
                    action.do_not_approve();
                }
            }
        }
    }
}

pub(crate) fn build_actions(mut building_action_data: ResMut<BuildingActions>) {
    for building_action in building_action_data.list.iter_mut() {
        match &building_action.target_cell_option {
            Some(_examined_entity) => {
                let mut new_vec = vec![
                    ActionData {
                        data: Action {
                            id: "action::construction_tool_admin/construct".to_string(),
                            text: "Construct".to_string(),
                            tab_list_priority: 50,
                        },
                        approved: None,
                    },
                    ActionData {
                        data: Action {
                            id: "action::construction_tool_admin/deconstruct".to_string(),
                            text: "Deconstruct".to_string(),
                            tab_list_priority: 49,
                        },
                        approved: None,
                    },
                    ActionData {
                        data: Action {
                            id: "action::construction_tool_admin/constructionoptions".to_string(),
                            text: "Construction Options".to_string(),
                            tab_list_priority: 48,
                        },
                        approved: None,
                    },
                ];

                building_action.actions.append(&mut new_vec);
            }
            None => {}
        }
    }
}
use crate::construction_tool::InputConstructionOptionsSelection;
use ui::text_input::TextTreeInputSelection;

pub(crate) fn text_tree_input_selection(
    mut input_events: EventReader<TextTreeInputSelection>,
    mut input_construction_options_selection: EventWriter<InputConstructionOptionsSelection>,
) {
    for event in input_events.iter() {
        let belonging_entity;
        match event.belonging_entity {
            Some(bits) => {
                belonging_entity = Some(Entity::from_bits(bits));
            }
            None => {
                belonging_entity = None;
            }
        }

        if event.menu_id == "textselection::construction_tool_admin/constructionoptionslist"
            && belonging_entity.is_some()
        {
            input_construction_options_selection.send(InputConstructionOptionsSelection {
                handle_option: Some(event.handle),
                menu_selection: event.menu_selection.clone(),
                entity: belonging_entity.unwrap(),
            });
        }
    }
}
