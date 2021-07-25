use std::collections::HashMap;

use bevy::prelude::{FromWorld, World};

use super::precalculated_fov_data::Vec2Int;


pub struct WorldFOV {
    pub data: HashMap<Vec2Int, Vec<Vec2Int>>,
    pub to_be_recalculated : Vec<Vec2Int>,
    pub to_be_recalculated_priority : Vec<Vec2Int>,
    pub init : bool,
    pub blocking_load_at_init : bool,
}

impl FromWorld for WorldFOV {
    fn from_world(_world: &mut World) -> Self {
        WorldFOV {
            data : HashMap::new(),
            to_be_recalculated: vec![],
            to_be_recalculated_priority: vec![],
            init: true,
            blocking_load_at_init: false,
        }
    }
}
