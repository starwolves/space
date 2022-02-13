use std::collections::{HashMap};



use crate::space::core::networking::resources::EntityUpdateData;

use super::match_entity_data::entity_data_is_matching;

pub fn entity_update_changed_detection(
    changed_parameters : &mut Vec<String>,
    entity_updates : &mut HashMap<String, EntityUpdateData>,
    set : EntityUpdateData,
    parameter : String,
) {
    let get = entity_updates.get(&parameter);
    let has_changed ;
    match get {
        Some(value) => {
            has_changed = !entity_data_is_matching(value, &set);
        },
        None => {
            has_changed=true;
        },
    }


    if has_changed == true {

        entity_updates.insert(
            parameter.clone(),
            set
        );
        changed_parameters.push(parameter);
    }

}
