use std::collections::HashMap;

use crate::space::core::{
    entity::components::EntityUpdates, networking::resources::EntityUpdateData,
};

pub fn personalise(
    updates_data: &mut HashMap<String, HashMap<String, EntityUpdateData>>,
    player_handle: u64,
    entity_updates_component: &EntityUpdates,
) {
    let mut to_be_removed_parameters = vec![];

    for key_value in entity_updates_component.excluded_handles.clone() {
        if updates_data.contains_key(&key_value.0) && key_value.1.contains(&player_handle) {
            to_be_removed_parameters.push(key_value.0);
        }
    }

    for parameter in to_be_removed_parameters {
        updates_data.remove(&parameter);
    }
}
