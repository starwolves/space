use std::collections::HashMap;

use bevy::prelude::{Commands, Entity, EventWriter, FromWorld, ResMut, Transform, World};

use crate::space_core::{components::{connected_player::ConnectedPlayer, persistent_player_data::PersistentPlayerData}, events::net::net_showcase::NetShowcase};

use super::used_names::UsedNames;

pub struct EntityDataResource {
    pub data : Vec<EntityDataProperties>,
    pub incremented_id : usize,
    pub id_to_name : HashMap<usize, String>,
    pub name_to_id : HashMap<String, usize>,
}

impl EntityDataResource {
    pub fn get_id_inc(&mut self) -> usize {
        let return_val = self.incremented_id.clone();
        self.incremented_id+=1;
        return_val
    }
}

impl FromWorld for EntityDataResource {
    fn from_world(_world: &mut World) -> Self {
        EntityDataResource {
            data : vec![],
            incremented_id: 0,
            id_to_name : HashMap::new(),
            name_to_id : HashMap::new(),
        }
    }
}

pub struct SpawnPawnData<'a, 'b> {
    pub data: (
        &'a PersistentPlayerData,
        Option<&'a ConnectedPlayer>,
        Vec<(String,String)>,
        bool,
        bool,
        Option<&'a mut ResMut<'b, UsedNames>>,
        Option<&'a mut EventWriter<'b, NetShowcase>>,
        Option<String>,
        &'a ResMut<'a, EntityDataResource>,
    )
}

pub struct SpawnHeldData<'a, 'b, 'c> {
    pub data: (
        Entity,
        bool,
        Option<u32>,
        &'c mut Option<&'b mut EventWriter<'a, NetShowcase>>,
    )
}

pub struct EntityDataProperties {
    pub spawn_function: Box<dyn Fn(
        Transform,
        &mut Commands,
        bool,
        Option<SpawnPawnData>,
        Option<SpawnHeldData>,
    ) -> Entity + Sync + Send>,
    pub name : String,
    pub id : usize,
}

impl Default for EntityDataProperties {
    fn default() -> Self {
        Self {
            spawn_function: Box::new(
                |
                _,
                _,
                _,
                _,
                _,
                | {
                    Entity::new(0)
                }
            ),
            name: Default::default(),
            id: Default::default()
        }
    }
}
