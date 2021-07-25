use bevy::prelude::{FromWorld, World};

pub struct UsedNames {
    pub names : Vec<String>
}

impl FromWorld for UsedNames {
    fn from_world(_world: &mut World) -> Self {
        UsedNames {
            names : vec![]
        }
    }
}
