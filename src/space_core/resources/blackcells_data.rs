use bevy::prelude::{FromWorld, World};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct BlackcellsData {
    pub blackcell_id : i64,
    pub blackcell_blocking_id : i64
}

impl FromWorld for BlackcellsData {
    fn from_world(_world: &mut World) -> Self {
        BlackcellsData {
            blackcell_id : 0,
            blackcell_blocking_id : 0,
        }
    }
}
