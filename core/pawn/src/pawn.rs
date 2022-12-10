use bevy::{math::Vec2, prelude::Component};

/// Ship authorizations for pawns.
#[derive(PartialEq)]
#[cfg(feature = "server")]
pub enum ShipAuthorizationEnum {
    Security,
    Common,
}
/// Crew jobs for pawns.
#[derive(Copy, Clone)]
#[cfg(feature = "server")]
pub enum ShipJobsEnum {
    Security,
    Control,
}
/// The component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Pawn {
    pub name: String,
    pub job: ShipJobsEnum,
    pub communicator: Communicator,
    pub facing_direction: FacingDirection,
}

/// The kind of communicator.
#[derive(Clone)]
#[cfg(feature = "server")]
pub enum Communicator {
    Standard,
    Machine,
}

#[cfg(feature = "server")]
impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: ShipJobsEnum::Security,
            facing_direction: FacingDirection::Up,
            communicator: Communicator::Standard,
        }
    }
}

/// Ship authorization component.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct ShipAuthorization {
    pub access: Vec<ShipAuthorizationEnum>,
}

#[derive(Debug, Clone)]
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
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
use bevy_rapier3d::na::Quaternion;

#[cfg(feature = "server")]
pub struct PawnYAxisRotations;

#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub const REACH_DISTANCE: f32 = 3.;
