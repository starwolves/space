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
