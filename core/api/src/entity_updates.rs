use std::collections::HashMap;

use bevy::{
    math::{Quat, Vec2, Vec3},
    prelude::Component,
};
use serde::{Deserialize, Serialize};

use crate::network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage};

pub fn entity_update_changed_detection(
    changed_parameters: &mut Vec<String>,
    entity_updates: &mut HashMap<String, EntityUpdateData>,
    set: EntityUpdateData,
    parameter: String,
) {
    let get = entity_updates.get(&parameter);
    let has_changed;
    match get {
        Some(value) => {
            has_changed = !entity_data_is_matching(value, &set);
        }
        None => {
            has_changed = true;
        }
    }

    if has_changed == true {
        entity_updates.insert(parameter.clone(), set);
        changed_parameters.push(parameter);
    }
}

pub struct NetSendEntityUpdates {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetSendEntityUpdates {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}
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

pub fn get_entity_update_difference(
    old_data: HashMap<String, HashMap<String, EntityUpdateData>>,
    new_data: &HashMap<String, HashMap<String, EntityUpdateData>>,
) -> HashMap<String, HashMap<String, EntityUpdateData>> {
    let mut difference_data = HashMap::new();

    for (new_node_path, new_data_entity_updates) in new_data {
        match old_data.get(new_node_path) {
            Some(old_data_entity_updates) => {
                for (new_entity_update_type, new_entity_update_data) in new_data_entity_updates {
                    match old_data_entity_updates.get(new_entity_update_type) {
                        Some(old_entity_update_data) => {
                            if !entity_data_is_matching(
                                new_entity_update_data,
                                old_entity_update_data,
                            ) {
                                if !difference_data.contains_key(&new_node_path.to_string()) {
                                    difference_data
                                        .insert(new_node_path.to_string(), HashMap::new());
                                }
                                let difference_data_entity_updates =
                                    difference_data.get_mut(&new_node_path.to_string()).unwrap();
                                difference_data_entity_updates.insert(
                                    new_entity_update_type.clone(),
                                    new_entity_update_data.clone(),
                                );
                            }
                        }
                        None => {
                            if !difference_data.contains_key(&new_node_path.to_string()) {
                                difference_data.insert(new_node_path.to_string(), HashMap::new());
                            }
                            let difference_data_entity_updates =
                                difference_data.get_mut(&new_node_path.to_string()).unwrap();
                            difference_data_entity_updates.insert(
                                new_entity_update_type.clone(),
                                new_entity_update_data.clone(),
                            );
                        }
                    }
                }
            }
            None => {
                difference_data.insert(new_node_path.to_string(), new_data_entity_updates.clone());
            }
        }
    }

    difference_data
}
#[derive(Component)]
pub struct EntityData {
    pub entity_class: String,
    pub entity_name: String,
    pub entity_group: EntityGroup,
}

impl Default for EntityData {
    fn default() -> Self {
        Self {
            entity_class: "".to_string(),
            entity_name: "".to_string(),
            entity_group: EntityGroup::None,
        }
    }
}

#[derive(Copy, Clone)]
pub enum EntityGroup {
    None,
    AirLock,
    CounterWindowSensor,
    Pawn,
}

pub fn entity_data_is_matching(data1: &EntityUpdateData, data2: &EntityUpdateData) -> bool {
    let mut is_not_matching = true;

    match data1 {
        EntityUpdateData::Int(old_value) => match data2 {
            EntityUpdateData::Int(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::UInt8(old_value) => match data2 {
            EntityUpdateData::UInt8(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::String(old_value) => match data2 {
            EntityUpdateData::String(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::StringVec(old_value) => match data2 {
            EntityUpdateData::StringVec(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Float(old_value) => match data2 {
            EntityUpdateData::Float(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Transform(old_value, old_value1, old_value2) => match data2 {
            EntityUpdateData::Transform(new_value, new_value1, new_value2) => {
                is_not_matching = *new_value != *old_value
                    || *old_value1 != *new_value1
                    || *old_value2 != *new_value2;
            }
            _ => {}
        },
        EntityUpdateData::Color(r, g, b, a) => match data2 {
            EntityUpdateData::Color(r_n, g_n, b_n, a_n) => {
                is_not_matching = r != r_n && g != g_n && b != b_n && a != a_n;
            }
            _ => {}
        },
        EntityUpdateData::Bool(old_value) => match data2 {
            EntityUpdateData::Bool(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::Vec3(old_value) => match data2 {
            EntityUpdateData::Vec3(new_value) => {
                is_not_matching = *new_value != *old_value;
            }
            _ => {}
        },
        EntityUpdateData::AttachedItem(old_value0, old_value1, old_value2, old_value3) => {
            match data2 {
                EntityUpdateData::AttachedItem(new_value0, new_value1, new_value2, new_value3) => {
                    is_not_matching = *new_value0 != *old_value0
                        || *new_value1 != *old_value1
                        || *new_value2 != *old_value2
                        || *new_value3 != *old_value3;
                }
                _ => {}
            }
        }
        EntityUpdateData::WornItem(
            old_value0,
            old_value1,
            old_value2,
            old_value3,
            old_value4,
            old_value5,
        ) => match data2 {
            EntityUpdateData::WornItem(
                new_value0,
                new_value1,
                new_value2,
                new_value3,
                new_value4,
                new_value5,
            ) => {
                is_not_matching = *new_value0 != *old_value0
                    || *new_value1 != *old_value1
                    || *new_value2 != *old_value2
                    || *new_value3 != *old_value3
                    || *new_value4 != *old_value4
                    || *new_value5 != *old_value5;
            }
            _ => {}
        },
        EntityUpdateData::WornItemNotAttached(old_value0, old_value1, old_value2) => match data2 {
            EntityUpdateData::WornItemNotAttached(new_value0, new_value1, new_value2) => {
                is_not_matching = *new_value0 != *old_value0
                    || *new_value1 != *old_value1
                    || *new_value2 != *old_value2;
            }
            _ => {}
        },
        EntityUpdateData::Vec2(old_value0) => match data2 {
            EntityUpdateData::Vec2(new_value0) => is_not_matching = *new_value0 != *old_value0,
            _ => {}
        },
    }

    !is_not_matching
}
/// Entity update component containing Godot node related updates for clients for visual changes.
#[derive(Component)]
pub struct EntityUpdates {
    pub updates: HashMap<String, HashMap<String, EntityUpdateData>>,
    pub updates_difference: Vec<HashMap<String, HashMap<String, EntityUpdateData>>>,
    pub changed_parameters: Vec<String>,
    pub excluded_handles: HashMap<String, Vec<u64>>,
}

impl Default for EntityUpdates {
    fn default() -> Self {
        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());
        Self {
            updates: entity_updates_map,
            changed_parameters: vec![],
            excluded_handles: HashMap::new(),
            updates_difference: vec![],
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EntityUpdateData {
    Int(i64),
    UInt8(u8),
    String(String),
    StringVec(Vec<String>),
    Float(f32),
    Transform(Vec3, Quat, Vec3),
    Color(f32, f32, f32, f32),
    Bool(bool),
    Vec3(Vec3),
    Vec2(Vec2),
    AttachedItem(u64, Vec3, Quat, Vec3),
    WornItem(String, u64, String, Vec3, Quat, Vec3),
    WornItemNotAttached(String, u64, String),
}
