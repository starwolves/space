use crate::space_core::enums::space_jobs::SpaceJobsEnum;

pub struct Pawn {

    pub name : String,
    pub job : SpaceJobsEnum,
    pub facing_direction : FacingDirection,

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
