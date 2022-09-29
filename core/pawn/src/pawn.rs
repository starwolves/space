use api::data::Vec3Int;
use bevy::{
    math::Vec2,
    prelude::{Component, Entity},
};

/// Ship authorizations for pawns.
#[derive(PartialEq)]
pub enum ShipAuthorizationEnum {
    Security,
    Common,
}
/// Crew jobs for pawns.
#[derive(Copy, Clone)]
pub enum ShipJobsEnum {
    Security,
    Control,
}
/// The component.
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

/// Ship authorization component.
#[derive(Component)]
pub struct ShipAuthorization {
    pub access: Vec<ShipAuthorizationEnum>,
}

/// Controller input component.
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

/// Persistent player data component.
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

/// Facing direction to Vec2 as a function.
pub fn facing_direction_to_direction(direction: &FacingDirection) -> Vec2 {
    match direction {
        FacingDirection::UpLeft => Vec2::new(-1., 1.),
        FacingDirection::Up => Vec2::new(0., 1.),
        FacingDirection::UpRight => Vec2::new(1., 1.),
        FacingDirection::Right => Vec2::new(1., 0.),
        FacingDirection::DownRight => Vec2::new(1., -1.),
        FacingDirection::Down => Vec2::new(0., -1.),
        FacingDirection::DownLeft => Vec2::new(-1., -1.),
        FacingDirection::Left => Vec2::new(-1., 0.),
    }
}
