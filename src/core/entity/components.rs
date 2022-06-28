use std::collections::HashMap;

use bevy_ecs::prelude::Component;

use crate::core::networking::resources::EntityUpdateData;

#[derive(Component)]
pub struct EntityData {
    pub entity_class: String,
    pub entity_name: String,
    pub entity_group: EntityGroup,
}

#[derive(Copy, Clone)]
pub enum EntityGroup {
    None,
    AirLock,
    CounterWindowSensor,
    Pawn,
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

#[derive(Component)]
pub struct Server;

#[derive(Component)]
pub struct Showcase {
    pub handle: u64,
}

#[derive(Component)]
pub struct DefaultMapEntity;
