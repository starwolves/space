
use std::collections::HashMap;

use bevy::{math::Vec2};

use crate::space_core::{functions::entity::get_tab_action::get_tab_action, resources::{network_messages::{GridMapType, NetTabAction}}};

pub struct TabAction {
    pub id : String,
    pub text : String,
    pub tab_list_priority : u8,
    pub prerequisite_check : Box<dyn Fn(
        Option<u64>,
        Option<(GridMapType, i16,i16,i16)>,
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

pub struct Pawn {

    pub name : String,
    pub job : SpaceJobsEnum,
    pub facing_direction : FacingDirection,
    pub tab_actions : HashMap<u16, TabAction>,
}

impl Default for Pawn {
    fn default() -> Self {
        let mut s = HashMap::new();
        s.insert(0,get_tab_action("examine").unwrap());
        Self {
            name: "".to_string(),
            job: SpaceJobsEnum::Security,
            facing_direction: FacingDirection::Up,
            tab_actions : s,
        }
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
