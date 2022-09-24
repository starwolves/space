use actions::data::{Action, ActionData, BuildingActions};
use bevy::prelude::{Query, ResMut};

use crate::item::InventoryItem;

pub fn build_actions(
    mut building_action_data: ResMut<BuildingActions>,
    inventory_items: Query<&InventoryItem>,
) {
    for building_action in building_action_data.list.iter_mut() {
        match building_action.target_entity_option {
            Some(examined_entity) => match inventory_items.get(examined_entity) {
                Ok(_) => {
                    let mut new_vec = vec![ActionData {
                        data: Action {
                            id: "actions::inventory/pickup".to_string(),
                            text: "Pickup".to_string(),
                            tab_list_priority: u8::MAX - 1,
                        },
                        approved: None,
                    }];

                    building_action.actions.append(&mut new_vec);
                }
                Err(_rr) => {}
            },
            None => {}
        }
    }
}
