use bevy::math::Vec3;
use bevy::prelude::{Entity, Transform};
use serde::{Deserialize, Serialize};

use crate::chat::ASTRIX;
use crate::data::{Vec2Int, Vec3Int};

use crate::health::{CellUpdate, Health};
use std::collections::HashMap;

/// Data stored in a resource of a cell instead of each cell having their own entity with components.
#[derive(Clone, Default)]
pub struct CellData {
    /// Cell item id.
    pub item: i64,
    /// Cell rotation.
    pub orientation: i64,
    /// The health of the cell.
    pub health: Health,
    /// Entity id if cell is an entity.
    pub entity: Option<Entity>,
}

/// Stores the details 1 gridmap layer, huge map data resource. In favor of having each ordinary tile having its own entity with its own sets of components.
#[derive(Default)]
pub struct GridmapDetails1 {
    pub grid_data: HashMap<Vec3Int, CellData>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
}

/// Turning up these values drastically increases fov calculation time.
/// The largest maps we can support with f32 accuracy is a 2000x2000 tiled map.
/// FOV calculation time will take 10x-15x slower, up to 2-3ms for just a single player calculation.
/// For bigger maps than 500x500 gridmaps we need a new and better FOV algorithm.
/// Dividible by 2.
pub const FOV_MAP_WIDTH: usize = 500;

/// Use this to use the Doryen FOV algorithm.
pub fn to_doryen_coordinates(x: i16, y: i16) -> (usize, usize) {
    let mut n_x = x + FOV_MAP_WIDTH as i16 / 2;
    let mut n_y = y + FOV_MAP_WIDTH as i16 / 2;

    if doryen_coordinates_out_of_range(n_x as usize, n_y as usize) {
        n_x = 0;
        n_y = 0;
    }

    (n_x as usize, n_y as usize)
}

/// Check if supplied doryen coordinates are out of range as a function.
pub fn doryen_coordinates_out_of_range(x: usize, y: usize) -> bool {
    x > FOV_MAP_WIDTH || y > FOV_MAP_WIDTH
}
pub const CELL_SIZE: f32 = 2.;

/// Use this to obtain data from large gridmap layer resources.
pub fn world_to_cell_id(position: Vec3) -> Vec3Int {
    let map_pos = position / CELL_SIZE;

    Vec3Int {
        x: map_pos.x.floor() as i16,
        y: map_pos.y.floor() as i16,
        z: map_pos.z.floor() as i16,
    }
}
/// Stores the main gridmap layer data, huge map data resource. In favor of having each ordinary tile having its own entity with its own sets of components.
#[derive(Default)]
pub struct GridmapMain {
    pub grid_data: HashMap<Vec3Int, CellData>,
    pub entity_data: HashMap<Vec3Int, EntityGridData>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
}

/// For entities that are also registered in the gridmap. (entity tiles)
pub struct EntityGridData {
    pub entity: Entity,
    pub entity_name: String,
}
/// Directional rotations alongside their "orientation" value used for Godot gridmaps.
#[derive(Clone)]
pub struct GridDirectionRotations {
    pub data: HashMap<AdjacentTileDirection, u8>,
}

impl GridDirectionRotations {
    pub fn default_wall_rotations() -> Self {
        let mut data = HashMap::new();
        data.insert(AdjacentTileDirection::Left, 23);
        data.insert(AdjacentTileDirection::Right, 19);
        data.insert(AdjacentTileDirection::Up, 14);
        data.insert(AdjacentTileDirection::Down, 4);
        Self { data }
    }
}
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum AdjacentTileDirection {
    Up,
    Down,
    Left,
    Right,
}
const Y_CENTER_OFFSET: f32 = 1.;

/// From tile id to world position.
pub fn cell_id_to_world(cell_id: Vec3Int) -> Vec3 {
    let mut world_position: Vec3 = Vec3::ZERO;

    world_position.x = (cell_id.x as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z as f32 * CELL_SIZE) + Y_CENTER_OFFSET;

    world_position
}

/// Stores examine messages being built this frame for gridmap examination.
#[derive(Default)]
pub struct GridmapExamineMessages {
    pub messages: Vec<ExamineMapMessage>,
}
/// Examine map message event.
pub struct ExamineMapMessage {
    pub handle: u64,
    pub entity: Entity,
    pub gridmap_type: GridMapLayer,
    pub gridmap_cell_id: Vec3Int,
    /// Map examine message being built and sent back to the player.
    pub message: String,
}
impl Default for ExamineMapMessage {
    fn default() -> Self {
        Self {
            handle: 0,
            entity: Entity::from_bits(0),
            gridmap_type: GridMapLayer::Main,
            gridmap_cell_id: Vec3Int::default(),
            message: ASTRIX.to_string(),
        }
    }
}
pub fn get_atmos_index(id: Vec2Int) -> usize {
    let idx: u32 = (id.x + (FOV_MAP_WIDTH / 2) as i16) as u32;
    let idy: u32 = (id.y + (FOV_MAP_WIDTH / 2) as i16) as u32;

    (idx + (idy * FOV_MAP_WIDTH as u32)) as usize
}

pub fn get_atmos_id(i: usize) -> Vec2Int {
    let y = (i as f32 / FOV_MAP_WIDTH as f32).floor() as usize;
    let x = i - (y * FOV_MAP_WIDTH);

    Vec2Int {
        x: x as i16 - (FOV_MAP_WIDTH as i16 / 2),
        y: y as i16 - (FOV_MAP_WIDTH as i16 / 2),
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum GridMapLayer {
    Main,
    Details1,
}

/// For entities that are also registered with the gridmap.
pub struct GridItemData {
    pub transform_offset: Transform,
    /// So this entity can be built on a cell when another item is already present on that cell.
    pub can_be_built_with_grid_item: Vec<String>,
}

/// Remove gridmap cell event.
pub struct RemoveCell {
    pub handle_option: Option<u64>,
    pub gridmap_type: GridMapLayer,
    pub id: Vec3Int,
    pub cell_data: CellData,
}
