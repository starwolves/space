use bevy::prelude::{FromWorld, World};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct NonBlockingCellsList {
    pub list: Vec<i64>
}

impl FromWorld for NonBlockingCellsList {
    fn from_world(_world: &mut World) -> Self {
        NonBlockingCellsList {
            list : vec![],
        }
    }
}
