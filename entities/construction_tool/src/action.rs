use actions::core::{Action, ActionData, ActionRequests, BuildingActions};
use bevy::prelude::{warn, EventReader, EventWriter, Query, Res, ResMut, With};
use gridmap::grid::Gridmap;
use inventory::item::InventoryItem;

use crate::construction_tool::{ConstructionTool, InputConstructionOptions, InputDeconstruct};
use networking::server::{HandleToEntity, OutgoingReliableServerMessage};

use super::construction_tool::InputConstruct;

/// Manage construction actions.

pub const CONSTRUCTION_ACTION_ID: &str = "action::construction_tool_admin/construct";
pub const DECONSTRUCTION_ACTION_ID: &str = "action::construction_tool_admin/deconstruct";
pub const CONSTRUCTION_OPTIONS_ACTION_ID: &str =
    "action::construction_tool_admin/constructionoptions";
pub const CONSTRUCTION_OPTIONS_TEXT_LIST_ID: &str = "ui::construction_tool_admin/selectionoptions";

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
                && action_data.data.id == CONSTRUCTION_ACTION_ID
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
                && action_data.data.id == DECONSTRUCTION_ACTION_ID
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
                && action_data.data.id == CONSTRUCTION_OPTIONS_ACTION_ID
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
                match building.target_entity_option {
                    Some(r) => {
                        event_construction_options.send(InputConstructionOptions {
                            handle_option,
                            entity: r,
                        });
                    }
                    None => {}
                }
            }
        }
    }
}

pub fn open_input_construction_options_ui(
    mut events: EventReader<InputConstructionOptions>,
    mut net: EventWriter<OutgoingReliableServerMessage<UiServerMessage>>,
    gridmap: Res<Gridmap>,
) {
    for event in events.iter() {
        match event.handle_option {
            Some(handle) => {
                net.send(OutgoingReliableServerMessage {
                    handle: handle,
                    message: UiServerMessage::TextTreeSelection(TextTreeSelection {
                        entity: event.entity,
                        id: CONSTRUCTION_OPTIONS_TEXT_LIST_ID.to_string(),
                        entries: gridmap.ordered_main_names.clone(),
                        text: "Select Construction".to_string(),
                    }),
                });
            }
            None => {}
        }
    }
}

pub fn construct_action_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    gridmap_main: Res<Gridmap>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == CONSTRUCTION_ACTION_ID {
                let cell_id;

                match building.target_cell_option.clone() {
                    Some(c) => {
                        cell_id = c;
                    }
                    None => {
                        continue;
                    }
                }

                let cell_option = gridmap_main.get_cell(cell_id.id, cell_id.face);

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
    gridmap_main: Res<Gridmap>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == DECONSTRUCTION_ACTION_ID {
                let cell_id;

                match building.target_cell_option.clone() {
                    Some(c) => {
                        cell_id = c;
                    }
                    None => {
                        continue;
                    }
                }

                let cell_option = gridmap_main.get_cell(cell_id.id, cell_id.face);

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

pub(crate) fn construction_tool_inventory_prequisite_check(
    mut building_action_data: ResMut<BuildingActions>,
    query: Query<&InventoryItem, With<ConstructionTool>>,
) {
    for building in building_action_data.list.iter_mut() {
        for action in building.actions.iter_mut() {
            if action.data.id == CONSTRUCTION_OPTIONS_ACTION_ID {
                match building.target_entity_option {
                    Some(tool_entity) => match query.get(tool_entity) {
                        Ok(inv) => match inv.in_inventory_of_entity {
                            Some(holder_entity) => {
                                if holder_entity == building.action_taker {
                                    action.approve();
                                } else {
                                    action.do_not_approve();
                                }
                            }
                            None => {}
                        },
                        Err(_) => {
                            warn!("Couldnt find action taker pawn.");
                        }
                    },
                    None => {}
                }
            }
        }
    }
}

pub(crate) fn build_actions(mut building_action_data: ResMut<BuildingActions>) {
    for building_action in building_action_data.list.iter_mut() {
        let mut new_vec = vec![
            ActionData {
                data: Action {
                    id: CONSTRUCTION_ACTION_ID.to_string(),
                    text: "Construct".to_string(),
                    tab_list_priority: 50,
                },
                approved: None,
            },
            ActionData {
                data: Action {
                    id: DECONSTRUCTION_ACTION_ID.to_string(),
                    text: "Deconstruct".to_string(),
                    tab_list_priority: 49,
                },
                approved: None,
            },
            ActionData {
                data: Action {
                    id: CONSTRUCTION_OPTIONS_ACTION_ID.to_string(),
                    text: "Construction Options".to_string(),
                    tab_list_priority: 48,
                },
                approved: None,
            },
        ];

        building_action.actions.append(&mut new_vec);
    }
}
use ui::{
    net::{TextTreeSelection, UiServerMessage},
    text_input::TextTreeInputSelection,
};

pub(crate) fn construction_tool_select_construction_option(
    mut input_events: EventReader<TextTreeInputSelection>,
    mut query: Query<&mut ConstructionTool>,
) {
    for event in input_events.iter() {
        if event.id == CONSTRUCTION_OPTIONS_TEXT_LIST_ID {
            match query.get_mut(event.entity) {
                Ok(mut c) => {
                    c.construction_option = Some(event.entry.clone());
                }
                Err(_) => {
                    warn!("Couldnt find construction tool {:?}.", event.entity);
                }
            }
        }
    }
}
