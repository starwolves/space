use actions::data::{Action, ActionData, BuildingActions, ListActionDataRequests};
use api::{
    examinable::Examinable,
    gridmap::{GridmapData, GridmapDetails1, GridmapMain},
};
use bevy::prelude::{warn, Query, Res, ResMut};

pub fn build_actions(
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

pub fn set_action_header_name(
    mut building_action_data: ResMut<BuildingActions>,
    examinables: Query<&Examinable>,
    gridmap_data: Res<GridmapData>,
    gridmap_main: Res<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    mut action_data_requests: ResMut<ListActionDataRequests>,
) {
    for building in building_action_data.list.iter_mut() {
        let action_data_request;

        match action_data_requests.list.get_mut(&building.incremented_i) {
            Some(a) => {
                action_data_request = a;
            }
            None => {
                continue;
            }
        }

        match building.target_entity_option {
            Some(e) => match examinables.get(e) {
                Ok(examinable_component) => {
                    action_data_request.set_id(examinable_component.name.get_name().to_string());
                }
                Err(_) => {
                    warn!("Entity had no examinable component.");
                }
            },
            None => {
                let gridmap = building.target_cell_option.clone().unwrap();

                let names;
                let cell_data;

                match gridmap.1 {
                    api::gridmap::GridMapType::Main => {
                        names = gridmap_data.main_text_names.clone();
                        cell_data = gridmap_main.grid_data.clone();
                    }
                    api::gridmap::GridMapType::Details1 => {
                        names = gridmap_data.details1_text_names.clone();
                        cell_data = gridmap_details1.grid_data.clone();
                    }
                }

                let item_id;

                match cell_data.get(&gridmap.0) {
                    Some(data) => {
                        item_id = data.item;
                    }
                    None => {
                        warn!("Couldnt find item_id!");
                        continue;
                    }
                }

                action_data_request.set_id(names.get(&item_id).unwrap().get_name().to_string());
            }
        }
    }
}
