use bevy::prelude::{FromWorld, World};

#[derive(Clone)]
pub struct AllOrderedCells {
    pub main : Vec<String>,
    pub details1: Vec<String>
}


impl FromWorld for AllOrderedCells {
    fn from_world(_world: &mut World) -> Self {
        AllOrderedCells {
            main : vec![],
            details1 : vec![],
        }
    }
}
