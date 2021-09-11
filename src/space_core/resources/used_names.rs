use std::collections::HashMap;

use bevy::prelude::{Entity, FromWorld, World};

pub struct UsedNames {
    pub names : HashMap<String, Entity>,
    pub ooc_names : HashMap<String, Entity>,
    pub player_i : u32,
    pub dummy_i : u32,
}

impl FromWorld for UsedNames {
    fn from_world(_world: &mut World) -> Self {
        UsedNames {
            names : HashMap::new(),
            ooc_names: HashMap::new(),
            player_i: 0,
            dummy_i : 0,
        }
    }
}
