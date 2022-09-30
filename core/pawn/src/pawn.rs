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
use bevy::{math::Quat, prelude::Transform};
use bevy_rapier3d::na::Quaternion;
use data_converters::converters::string_transform_to_transform;
use math::grid::Vec3Int;
use serde::Deserialize;

pub struct PawnYAxisRotations;

impl PawnYAxisRotations {
    pub fn new() -> Vec<Quaternion<f32>> {
        vec![
            //0deg
            Quaternion::new(1., 0., 0., 0.),
            //45deg
            Quaternion::new(0.9238795, 0., 0.3826834, 0.),
            //90deg
            Quaternion::new(
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
            ),
            //135deg
            Quaternion::new(0.3826834, 0., 0.9238795, 0.),
            //180deg
            Quaternion::new(0., 0., 1., 0.),
            //225deg
            Quaternion::new(-0.3826834, 0., 0.9238795, 0.),
            //270deg
            Quaternion::new(
                -std::f32::consts::FRAC_1_SQRT_2,
                0.,
                std::f32::consts::FRAC_1_SQRT_2,
                0.,
            ),
            //315deg
            Quaternion::new(-0.9238795, 0., 0.3826834, 0.),
        ]
    }
}

#[derive(Clone)]
pub enum PawnDesignation {
    Showcase,
    Player,
    Dummy,
    Ai,
}
/// Component that contains the spawn data of a to-be-spawned entity.
#[derive(Component)]
pub struct Spawning {
    pub transform: Transform,
}

/// A spawn point in which players will spawn.
pub struct SpawnPoint {
    pub point_type: String,
    pub transform: Transform,
}

impl SpawnPoint {
    pub fn new(raw: &SpawnPointRaw) -> SpawnPoint {
        let mut this_transform = string_transform_to_transform(&raw.transform);

        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;

        SpawnPoint {
            point_type: raw.point_type.clone(),
            transform: this_transform,
        }
    }
}

/// Raw json.
#[derive(Deserialize)]
pub struct SpawnPointRaw {
    pub point_type: String,
    pub transform: String,
}
/// Resource containing all available spawn points for players.
#[derive(Default)]
pub struct SpawnPoints {
    pub list: Vec<SpawnPoint>,
    pub i: usize,
}
/// How far an entity can reach ie with picking up items.
pub const REACH_DISTANCE: f32 = 3.;
