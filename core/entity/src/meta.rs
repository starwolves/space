use std::collections::HashMap;

use bevy::prelude::Component;
use gridmap_meta::core::GridItemData;

/// Resource that contains entity meta data.
#[derive(Default)]
pub struct EntityDataResource {
    pub data: Vec<EntityDataProperties>,
    pub incremented_id: usize,
    pub id_to_name: HashMap<usize, String>,
    pub name_to_id: HashMap<String, usize>,
}

impl EntityDataResource {
    pub fn get_id_inc(&mut self) -> usize {
        let return_val = self.incremented_id.clone();
        self.incremented_id += 1;
        return_val
    }
}

impl Default for EntityDataProperties {
    fn default() -> Self {
        Self {
            name: Default::default(),
            id: Default::default(),
            grid_item: None,
        }
    }
}

/// Meta data for an entity.
pub struct EntityDataProperties {
    pub name: String,
    pub id: usize,
    pub grid_item: Option<GridItemData>,
}
/// The component for players that haven't yet boarded.
#[derive(Component)]
pub struct SoftPlayer;
