use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    prelude::{FromWorld, World},
    system::{Commands, ResMut},
};
use bevy_transform::components::Transform;

use crate::core::{
    connected_player::components::ConnectedPlayer,
    networking::resources::ConsoleCommandVariantValues,
    pawn::{components::PersistentPlayerData, resources::UsedNames},
};

use super::events::NetShowcase;

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

impl FromWorld for EntityDataResource {
    fn from_world(_world: &mut World) -> Self {
        EntityDataResource {
            data: vec![],
            incremented_id: 0,
            id_to_name: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }
}

pub enum PawnDesignation {
    Showcase,
    Player,
    Dummy,
    Ai,
}

pub struct SpawnPawnData<'a, 'b> {
    pub data: (
        &'a PersistentPlayerData,
        Option<&'a ConnectedPlayer>,
        Vec<(String, String)>,
        PawnDesignation,
        Option<&'a mut ResMut<'b, UsedNames>>,
        Option<String>,
        &'a ResMut<'a, EntityDataResource>,
    ),
}

pub struct SpawnHeldData {
    pub entity: Entity,
}

pub struct EntityDataProperties {
    pub spawn_function: Box<dyn Fn(SpawnData) -> Entity + Sync + Send>,
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
            spawn_function: Box::new(|_| Entity::from_raw(0)),
            name: Default::default(),
            id: Default::default(),
            grid_item: None,
        }
    }
}

pub struct ShowcaseData<'b, 'c, 'd> {
    pub handle: u32,
    pub event_writer: &'b mut EventWriter<'c, 'd, NetShowcase>,
}

pub struct SpawnData<'a, 'b, 'c, 'd, 'w, 's> {
    pub entity_transform: Transform,
    pub commands: &'a mut Commands<'w, 's>,
    pub correct_transform: bool,
    pub pawn_data_option: Option<SpawnPawnData<'a, 'b>>,
    pub held_data_option: Option<SpawnHeldData>,
    pub default_map_spawn: bool,
    pub properties: HashMap<String, ConsoleCommandVariantValues>,
    pub showcase_data_option: &'a mut Option<ShowcaseData<'b, 'c, 'd>>,
}
