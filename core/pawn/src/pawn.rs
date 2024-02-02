use bevy::{
    ecs::{entity::Entity, event::Event},
    math::Vec2,
    prelude::{Component, Transform},
};

/// Ship authorizations for pawns.
#[derive(PartialEq)]

pub enum ShipAuthorizationEnum {
    Security,
    Common,
}
/// Crew jobs for pawns.
#[derive(Default, Copy, Clone)]

pub enum ShipJobsEnum {
    #[default]
    Security,
    Control,
}
/// The component.
#[derive(Default, Component, Clone)]

pub struct Pawn {
    pub character_name: String,
    pub job: ShipJobsEnum,
    pub communicator: Communicator,
    pub facing_direction: FacingDirection,
}

/// The kind of communicator.
#[derive(Default, Clone)]

pub enum Communicator {
    #[default]
    Standard,
    Machine,
}
/// Ship authorization component.
#[derive(Component)]

pub struct ShipAuthorization {
    pub access: Vec<ShipAuthorizationEnum>,
}

#[derive(Default, Debug, Clone)]

pub enum FacingDirection {
    UpLeft,
    #[default]
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
use bevy_xpbd_3d::parry::na::Quaternion;
use entity::entity_types::EntityType;
use networking::server::ConnectedPlayer;

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

/// How far an entity can reach ie with picking up items.

pub const REACH_DISTANCE: f32 = 3.;

/// Data for spawning.
#[derive(Clone, Default)]

pub struct SpawnPawnData {
    pub pawn_component: Pawn,
    pub connected_player_option: Option<ConnectedPlayer>,
    pub designation: PawnDesignation,
    pub inventory_setup: Vec<Box<dyn EntityType>>,
}

#[derive(Clone, Default)]

pub enum PawnDesignation {
    Showcase,
    #[default]
    Player,
    Dummy,
    Ai,
}

/// Component that contains the spawn data of a to-be-spawned pawn.
#[derive(Event)]

pub struct SpawningPlayer {
    pub transform: Transform,
    pub entity: Entity,
}
/// How far melee fists attacks can reach.

pub const ARMS_REACH: f32 = 1.2;

/// The component for entities with data links.
#[derive(Component, Default)]

pub struct DataLink {
    pub links: Vec<DataLinkType>,
}

#[derive(PartialEq)]

pub enum DataLinkType {
    FullAtmospherics,
    RemoteLock,
    ShipEngineeringKnowledge,
}
pub trait PawnBuilder: Send + Sync {
    fn get_spawn_pawn_data(&self) -> SpawnPawnData;
}
