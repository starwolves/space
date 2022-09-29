use bevy::prelude::Transform;
use bevy::prelude::Vec3;

use crate::data::Vec3Int;

/// Use this to obtain data from large gridmap layer resources.
pub fn world_to_cell_id(position: Vec3) -> Vec3Int {
    let map_pos = position / CELL_SIZE;

    Vec3Int {
        x: map_pos.x.floor() as i16,
        y: map_pos.y.floor() as i16,
        z: map_pos.z.floor() as i16,
    }
}
/// Size of a cell.
pub const CELL_SIZE: f32 = 2.;

/// For entities that are also registered with the gridmap.
pub struct GridItemData {
    pub transform_offset: Transform,
    /// So this entity can be built on a cell when another item is already present on that cell.
    pub can_be_built_with_grid_item: Vec<String>,
}
