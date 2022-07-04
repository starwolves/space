use std::collections::HashMap;

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

use bevy::prelude::{Changed, Component, Entity, EventWriter, Query, Res};

use crate::core::{
    connected_player::{connection::ConnectedPlayer, plugin::HandleToEntity},
    networking::networking::{EntityUpdateData, EntityWorldType, ReliableServerMessage},
    sensable::sensable::Sensable,
};

use super::entity_data::{entity_data_is_matching, Showcase};

pub fn send_entity_updates(
    mut updated_entity_updates: Query<
        (
            Entity,
            Option<&Sensable>,
            &mut EntityUpdates,
            Option<&ConnectedPlayer>,
            Option<&Showcase>,
        ),
        Changed<EntityUpdates>,
    >,
    mut net_send_entity_updates: EventWriter<NetSendEntityUpdates>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for (
        visible_entity,
        visible_component_option,
        mut entity_updates_component,
        connected_player_component_option,
        showcase_component_option,
    ) in updated_entity_updates.iter_mut()
    {
        if entity_updates_component.changed_parameters.len() == 1
            && entity_updates_component
                .changed_parameters
                .contains(&"play_back_position".to_string())
        {
            entity_updates_component.updates_difference.clear();
            continue;
        }

        match visible_component_option {
            Some(visible_component) => {
                for sensed_by_entity in visible_component.sensed_by.iter() {
                    let mut updates_data_vec = entity_updates_component.updates_difference.clone();

                    for updates_data in updates_data_vec.iter_mut() {
                        match connected_player_component_option {
                            Some(connected_player_component) => {
                                personalise(
                                    updates_data,
                                    connected_player_component.handle,
                                    &entity_updates_component,
                                );
                            }
                            None => {}
                        }

                        if updates_data.len() == 0 {
                            continue;
                        }

                        match handle_to_entity.inv_map.get(&sensed_by_entity) {
                            Some(handle) => {
                                net_send_entity_updates.send(NetSendEntityUpdates {
                                    handle: *handle,
                                    message: ReliableServerMessage::EntityUpdate(
                                        visible_entity.to_bits(),
                                        updates_data.clone(),
                                        false,
                                        EntityWorldType::Main,
                                    ),
                                });
                            }
                            None => {}
                        }
                    }
                }

                entity_updates_component.updates_difference.clear();
            }
            None => {}
        }

        match showcase_component_option {
            Some(showcase_component) => {
                let mut updates_data = entity_updates_component.updates.clone();

                match connected_player_component_option {
                    Some(connected_player_component) => {
                        personalise(
                            &mut updates_data,
                            connected_player_component.handle,
                            &entity_updates_component,
                        );
                    }
                    None => {}
                }

                if updates_data.len() == 0 {
                    continue;
                }

                net_send_entity_updates.send(NetSendEntityUpdates {
                    handle: showcase_component.handle,
                    message: ReliableServerMessage::EntityUpdate(
                        visible_entity.to_bits(),
                        updates_data,
                        false,
                        EntityWorldType::Main,
                    ),
                });
            }
            None => {}
        }
    }
}

pub struct NetSendEntityUpdates {
    pub handle: u64,
    pub message: ReliableServerMessage,
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
