use std::collections::HashMap;

use bevy::prelude::Component;

use crate::space_core::resources::network_messages::EntityUpdateData;


#[derive(Component)]
pub struct EntityUpdates {
    pub updates : HashMap<String,HashMap<String, EntityUpdateData>>,
    pub updates_difference : Vec<HashMap<String,HashMap<String, EntityUpdateData>>>,
    pub changed_parameters : Vec<String>,
    pub excluded_handles : HashMap<String, Vec<u32>>
}


impl Default for EntityUpdates {
    fn default() -> Self {
        let mut entity_updates_map = HashMap::new();
        entity_updates_map.insert(".".to_string(), HashMap::new());
        Self {
            updates: entity_updates_map,
            changed_parameters: vec![],
            excluded_handles:HashMap::new(),
            updates_difference: vec![],
        }
    }
}
