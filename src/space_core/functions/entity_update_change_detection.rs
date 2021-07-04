use std::collections::{HashMap};


use crate::space_core::structs::network_messages::EntityUpdateData;

pub fn entity_update_changed_detection(
    changed_parameters : &mut Vec<String>,
    entity_updates : &mut HashMap<String, EntityUpdateData>,
    set : EntityUpdateData,
    parameter : String,
) {
    let get = entity_updates.get(&parameter);
    let mut has_changed = false;
    match get {
        Some(value) => {
            match value {
                EntityUpdateData::Int(old_value) => {
                    match set.clone() {
                        EntityUpdateData::Int(new_value) => {
                            has_changed = new_value != *old_value;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::UInt8(old_value) => {
                    match set.clone() {
                        EntityUpdateData::UInt8(new_value) => {
                            has_changed = new_value != *old_value;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::String(old_value) => {
                    match set.clone() {
                        EntityUpdateData::String(new_value) => {
                            has_changed = new_value != *old_value;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::StringVec(old_value) => {
                    match set.clone() {
                        EntityUpdateData::StringVec(new_value) => {
                            has_changed = new_value != *old_value;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::Float(old_value) => {
                    match set.clone() {
                        EntityUpdateData::Float(new_value) => {
                            has_changed = new_value != *old_value;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::Transform(old_value, old_value1, old_value2) => {
                    match set.clone() {
                        EntityUpdateData::Transform(new_value,new_value1,new_value2) => {
                            has_changed = new_value != *old_value || *old_value1 != new_value1 || *old_value2 != new_value2;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::Color(old_value) => {
                    match set.clone() {
                        EntityUpdateData::Color(new_value) => {
                            has_changed = new_value != *old_value;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::Bool(old_value) => {
                    match set.clone() {
                        EntityUpdateData::Bool(new_value) => {
                            has_changed = new_value != *old_value;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::Vec3(old_value) => {
                    match set.clone() {
                        EntityUpdateData::Vec3(new_value) => {
                            has_changed = new_value != *old_value;
                        },
                        _ => {}
                    }
                },
                EntityUpdateData::AttachedItem(old_value0, old_value1, old_value2, old_value3) => {
                    match set.clone() {
                        EntityUpdateData::AttachedItem(new_value0, new_value1,new_value2,new_value3) => {
                            has_changed = new_value0 != *old_value0
                            || new_value1 != *old_value1
                            || new_value2 != *old_value2
                            || new_value3 != *old_value3;
                        },
                        _ => {}
                    }
                },
            }
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
