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

#[derive(Copy, Clone)]
pub enum SpaceJobsEnum {

    Security,
    Control

}


#[derive(PartialEq)]
pub enum SpaceAccessEnum {
    Security
}
