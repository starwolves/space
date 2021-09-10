use bevy::prelude::{FromWorld, World};

pub struct UsedNames {
    pub names : Vec<String>,
    pub dummy_i : u16,
}

impl FromWorld for UsedNames {
    fn from_world(_world: &mut World) -> Self {
        UsedNames {
            names : vec![],
            dummy_i : 0,
        }
    }
}
