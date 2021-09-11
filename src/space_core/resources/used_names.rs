use std::collections::HashMap;

use bevy::prelude::{Entity, FromWorld, World};

pub struct UsedNames {
    pub names : HashMap<String, Entity>,
    pub dummy_i : u16,
}

impl FromWorld for UsedNames {
    fn from_world(_world: &mut World) -> Self {
        UsedNames {
            names : HashMap::new(),
            dummy_i : 0,
        }
    }
}
