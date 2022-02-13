use std::collections::{HashMap, BTreeMap};

use bevy::prelude::{Component, Entity, EventWriter, Res};

use crate::space::{core::{pawn::resources::HandleToEntity, networking::resources::EntityUpdateData}};

use super::{events::NetUnloadEntity, functions::unload_entity_for_player::unload_entity};

#[derive(Component)]
pub struct EntityData {
    pub entity_class : String,
    pub entity_type : String,
    pub entity_group : EntityGroup,
}


#[derive(Copy, Clone)]
pub enum EntityGroup {
    None,
    AirLock,
    CounterWindowSensor,
    Pawn
}

impl Default for EntityData {
    fn default() -> Self {
        Self {
            entity_class : "".to_string(),
            entity_type : "".to_string(),
            entity_group: EntityGroup::None,
        }
    }
}

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


#[derive(Component)]
pub struct Examinable {
    pub assigned_texts : BTreeMap<u32, String>,
    pub name : RichName,
}

impl Default for Examinable {
    fn default() -> Self {
        Self {
            assigned_texts : BTreeMap::new(),
            name: RichName::default(),
        }
    }
}

impl Default for RichName {
    fn default() -> Self {
        Self {
            name : "".to_string(),
            n : false,
            the : false,
        }
    }
}


#[derive(Clone, Debug)]
pub struct RichName {
    pub name : String,
    pub n : bool,
    pub the : bool,
}

impl RichName {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_a_name(&self) -> String {
        let prefix;
        if self.the {
            prefix = "the";
        } else {
            if self.n {
                prefix = "an";
            } else {
                prefix = "a";
            }
        }
        prefix.to_owned() + " " + &self.name
    }
}


#[derive(Component)]
pub struct Sensable{
    pub is_light : bool,
    pub is_audible : bool,
    pub sensed_by : Vec<Entity>,
    pub sensed_by_cached : Vec<Entity>,
    pub always_sensed : bool
}

impl Default for Sensable {
    fn default() -> Self {
        Self {
            is_audible : false,
            is_light:false,
            sensed_by_cached:vec![],
            sensed_by:vec![],
            always_sensed : false
        }
    }
}


impl Sensable {
    pub fn despawn(
        &mut self,
        entity : Entity,
        mut net_unload_entity : &mut EventWriter<NetUnloadEntity>,
        handle_to_entity : &Res<HandleToEntity>,
    ) {

        // Shouldn't be called from the same stage visible_checker.system() runs in.

        for sensed_by_entity in self.sensed_by.iter() {
            match handle_to_entity.inv_map.get(&sensed_by_entity) {
                Some(handle) => {
                    unload_entity(*handle, entity, &mut net_unload_entity, true);
                }
                None => {}
            }
        }
        for sensed_by_entity in self.sensed_by_cached.iter() {
            match handle_to_entity.inv_map.get(&sensed_by_entity) {
                Some(handle) => {
                    unload_entity(*handle, entity, &mut net_unload_entity, true);
                }
                None => {}
            }
        }

        self.sensed_by = vec![];
        self.sensed_by_cached = vec![];
    }
}


#[derive(Component)]
pub struct Server;

#[derive(Component)]
pub struct Showcase {
    pub handle : u32,
}
