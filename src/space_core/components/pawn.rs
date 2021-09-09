use bevy::math::Vec2;

pub struct Pawn {

    pub name : String,
    pub job : SpaceJobsEnum,
    pub facing_direction : FacingDirection,

}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: SpaceJobsEnum::Security,
            facing_direction: FacingDirection::Up,
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
