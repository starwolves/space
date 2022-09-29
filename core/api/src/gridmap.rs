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
