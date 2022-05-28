use std::collections::HashMap;

use crate::core::gridmap::resources::Vec2Int;

pub const GREEN_MAP_TILE_ENTRANCE: i16 = 3;
pub const GREEN_MAP_TILE_COUNTER: i16 = 4;

#[derive(Default)]
pub struct MapData {
    pub data: HashMap<Vec2Int, i16>,
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
