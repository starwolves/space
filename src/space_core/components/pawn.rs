use std::{collections::HashMap, sync::Arc};

use bevy::{math::Vec2, prelude::{Entity, Component}};

use crate::space_core::{resources::{network_messages::{GridMapType, NetTabAction}}};

use super::inventory::Inventory;


#[derive(Clone)]
pub struct TabAction {
    pub id : String,
    pub text : String,
    pub tab_list_priority : u8,
    pub prerequisite_check : Arc<dyn Fn(
        Option<u64>,
        Option<(GridMapType, i16,i16,i16)>,
        f32,
        &Inventory,
    ) -> bool + Sync + Send>,
}

impl TabAction {
    pub fn into_net(&self, item_name : &str, entity_option : Option<u64>, cell_option : Option<(GridMapType, i16,i16,i16)>) -> NetTabAction {
        NetTabAction {
            id: self.id.clone(),
            text: self.text.clone(),
            tab_list_priority: self.tab_list_priority,
            entity_option : entity_option,
            cell_option,
            item_name : item_name.to_string(),
        }
    }
}

#[derive(Component)]
pub struct Pawn {
    pub name : String,
    pub job : SpaceJobsEnum,
    pub facing_direction : FacingDirection,
    pub tab_actions : HashMap<u32, TabAction>,
    pub tab_actions_data : TabActionsData,
}

pub struct TabActionsData {
    pub layout : HashMap<Option<Entity>, HashMap<String, u32>>,
    pub tab_action_i : u32,
}

impl Default for TabActionsData {
    fn default() -> Self {
        Self {
            layout: HashMap::new(),
            tab_action_i:0,
        }
    }
}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: SpaceJobsEnum::Security,
            facing_direction: FacingDirection::Up,
            tab_actions : HashMap::new(),
            tab_actions_data: TabActionsData::default(),
        }
    }
}

impl Pawn {

    pub fn tab_actions_add(&mut self, tab_action_id : &str, entity_option : Option<Entity>, tab_action : TabAction) {
        
        let entity_tab_ids;

        match  self.tab_actions_data.layout.contains_key(&entity_option) {
            true => {
                entity_tab_ids = self.tab_actions_data.layout.get_mut(&entity_option).unwrap();
            },
            false => {
                self.tab_actions_data.layout.insert(entity_option, HashMap::new());
                entity_tab_ids = self.tab_actions_data.layout.get_mut(&entity_option).unwrap();
            },
        }

        entity_tab_ids.insert(tab_action_id.to_string(), self.tab_actions_data.tab_action_i);
        self.tab_actions.insert(self.tab_actions_data.tab_action_i, tab_action);
        self.tab_actions_data.tab_action_i+=1;
        
    }
    pub fn tab_actions_remove_entity(&mut self, entity_option : Option<Entity>) {

        let entity_tab_ids;

        match self.tab_actions_data.layout.get_mut(&entity_option) {
            Some(s) => {
                entity_tab_ids=s;
            },
            None => {
                return;
            },
        };

        for (_s, hashmap_index) in entity_tab_ids.iter() {
            self.tab_actions.remove(hashmap_index);
        }

        self.tab_actions_data.layout.remove(&entity_option);

    }

}

#[derive(Debug, Clone)]
pub enum FacingDirection {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

pub fn facing_direction_to_direction(direction : &FacingDirection) -> Vec2 {
    match direction {
        FacingDirection::UpLeft => {
            Vec2::new(-1.,1.)
        },
        FacingDirection::Up => {
            Vec2::new(0.,1.)
        },
        FacingDirection::UpRight => {
            Vec2::new(1. ,1.)
        },
        FacingDirection::Right => {
            Vec2::new(1., 0.)
        },
        FacingDirection::DownRight => {
            Vec2::new(1. , -1.)
        },
        FacingDirection::Down => {
            Vec2::new(0.,-1.)
        },
        FacingDirection::DownLeft => {
            Vec2::new(-1.,-1.)
        },
        FacingDirection::Left => {
            Vec2::new(-1.,0.)
        },
    }
}

#[derive(Copy, Clone)]
pub enum SpaceJobsEnum {
    Security,
    Control
}


#[derive(PartialEq)]
pub enum SpaceAccessEnum {
    Security,
    Common,
}
