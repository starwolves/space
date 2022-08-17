use std::collections::HashMap;

use api::{data::Vec3Int, get_spawn_position::FacingDirection};
use bevy::{
    math::Vec2,
    prelude::{Component, Entity},
};

#[derive(PartialEq)]
pub enum ShipAuthorizationEnum {
    Security,
    Common,
}
#[derive(Copy, Clone)]
pub enum ShipJobsEnum {
    Security,
    Control,
}
#[derive(Component)]
pub struct Pawn {
    pub name: String,
    pub job: ShipJobsEnum,
    pub facing_direction: FacingDirection,
}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: ShipJobsEnum::Security,
            facing_direction: FacingDirection::Up,
        }
    }
}

#[derive(Component)]
pub struct ShipAuthorization {
    pub access: Vec<ShipAuthorizationEnum>,
}

#[derive(Component)]
pub struct ControllerInput {
    pub movement_vector: Vec2,
    pub sprinting: bool,
    pub is_mouse_action_pressed: bool,
    pub targetted_limb: String,
    pub auto_move_enabled: bool,
    pub auto_move_direction: Vec2,
    pub combat_targetted_entity: Option<Entity>,
    pub combat_targetted_cell: Option<Vec3Int>,
    pub alt_attack_mode: bool,
    pub pending_direction: Option<FacingDirection>,
}
impl Default for ControllerInput {
    fn default() -> Self {
        Self {
            movement_vector: Vec2::ZERO,
            sprinting: false,
            is_mouse_action_pressed: false,
            targetted_limb: "torso".to_string(),
            auto_move_enabled: false,
            auto_move_direction: Vec2::ZERO,
            combat_targetted_entity: None,
            combat_targetted_cell: None,
            alt_attack_mode: false,
            pending_direction: None,
        }
    }
}
#[derive(Clone, Component)]
pub struct PersistentPlayerData {
    pub user_name_is_set: bool,
    pub character_name: String,
    pub user_name: String,
}
impl Default for PersistentPlayerData {
    fn default() -> Self {
        Self {
            user_name_is_set: false,
            character_name: "".to_string(),
            user_name: "".to_string(),
        }
    }
}

#[derive(Default, Clone)]
pub struct UsedNames {
    pub names: HashMap<String, Entity>,
    pub user_names: HashMap<String, Entity>,
    pub player_i: u32,
    pub dummy_i: u32,
}
pub fn get_dummy_name(used_names: &mut UsedNames) -> String {
    let return_name = format!("Dummy {}", used_names.dummy_i);

    used_names.dummy_i += 1;

    return_name
}
