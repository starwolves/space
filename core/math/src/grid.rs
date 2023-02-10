use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Default)]
pub struct Vec2Int {
    pub x: i16,
    pub y: i16,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Vec3Int {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

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
pub const CELL_SIZE: f32 = 1.;

const Y_CENTER_OFFSET: f32 = 0.5;

/// From tile id to world position.

pub fn cell_id_to_world(cell_id: Vec3Int) -> Vec3 {
    let mut world_position: Vec3 = Vec3::ZERO;

    world_position.x = (cell_id.x as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z as f32 * CELL_SIZE) + Y_CENTER_OFFSET;

    world_position
}
