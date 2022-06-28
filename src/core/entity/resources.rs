use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_transform::components::Transform;

use crate::core::{
    connected_player::components::ConnectedPlayer, pawn::components::PersistentPlayerData,
};

use super::functions::raw_entity::RawEntity;

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

#[derive(Clone)]
pub enum PawnDesignation {
    Showcase,
    Player,
    Dummy,
    Ai,
}

#[derive(Clone)]
pub struct SpawnPawnData {
    pub persistent_player_data: PersistentPlayerData,
    pub connected_player_option: Option<ConnectedPlayer>,
    pub inventory_setup: Vec<(String, String)>,
    pub designation: PawnDesignation,
}

pub struct EntityDataProperties {
    pub name: String,
    pub id: usize,
    pub grid_item: Option<GridItemData>,
}

pub struct GridItemData {
    pub transform_offset: Transform,
    pub can_be_built_with_grid_item: Vec<String>,
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

#[derive(Clone)]
pub struct ShowcaseData {
    pub handle: u64,
}

#[derive(Clone)]
pub struct SpawnData {
    pub entity_transform: Transform,
    pub correct_transform: bool,
    pub holder_entity_option: Option<Entity>,
    pub held_entity_option: Option<Entity>,
    pub default_map_spawn: bool,
    pub raw_entity_option: Option<RawEntity>,
    pub showcase_data_option: Option<ShowcaseData>,
    pub entity_name: String,
    pub entity: Entity,
}

impl Default for SpawnData {
    fn default() -> Self {
        Self {
            entity_transform: Transform::identity(),
            correct_transform: true,
            held_entity_option: None,
            holder_entity_option: None,
            default_map_spawn: false,
            raw_entity_option: None,
            showcase_data_option: None,
            entity_name: "".to_string(),
            entity: Entity::from_bits(0),
        }
    }
}
