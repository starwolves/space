use std::collections::HashMap;

use bevy_ecs::prelude::{FromWorld, World};

use crate::space::core::gridmap::resources::Vec2Int;

pub const GREEN_MAP_TILE_ENTRANCE: i16 = 3;
pub const GREEN_MAP_TILE_COUNTER: i16 = 4;

pub struct MapData {
    pub data: HashMap<Vec2Int, i16>,
}

impl FromWorld for MapData {
    fn from_world(_world: &mut World) -> Self {
        MapData {
            data: HashMap::new(),
        }
    }
}

impl MapData {
    pub fn to_net(&self) -> Vec<(i16, i16, i16)> {
        let mut net_data = vec![];

        for (id, item) in self.data.iter() {
            net_data.push((id.x, id.y, *item));
        }

        net_data
    }
}
