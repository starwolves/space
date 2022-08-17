use bevy::math::Vec3;
use bevy::prelude::{Entity, Res, Transform};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider};
use serde::{Deserialize, Serialize};

use crate::chat::ASTRIX;
use crate::data::{Vec2Int, Vec3Int};

#[derive(Clone, Default)]
pub struct CellData {
    pub item: i64,
    pub orientation: i64,
    pub health: Health,
    pub entity: Option<Entity>,
}

#[derive(Default)]
pub struct GridmapDetails1 {
    pub grid_data: HashMap<Vec3Int, CellData>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
}

// Turning up these values drastically increases fov calculation time.
// The largest maps we can support with f32 accuracy is a 2000x2000 tiled map.
// FOV calculation time will take 10x-15x slower, up to 2-3ms for just a single player calculation.
// For bigger maps than 500x500 gridmaps we need a new and better FOV algorithm.
// Dividible by 2.
pub const FOV_MAP_WIDTH: usize = 500;

pub fn to_doryen_coordinates(x: i16, y: i16) -> (usize, usize) {
    let mut n_x = x + FOV_MAP_WIDTH as i16 / 2;
    let mut n_y = y + FOV_MAP_WIDTH as i16 / 2;

    if doryen_coordinates_out_of_range(n_x as usize, n_y as usize) {
        n_x = 0;
        n_y = 0;
    }

    (n_x as usize, n_y as usize)
}

pub fn doryen_coordinates_out_of_range(x: usize, y: usize) -> bool {
    x > FOV_MAP_WIDTH || y > FOV_MAP_WIDTH
}
pub const CELL_SIZE: f32 = 2.;

pub fn world_to_cell_id(position: Vec3) -> Vec3Int {
    let map_pos = position / CELL_SIZE;

    Vec3Int {
        x: map_pos.x.floor() as i16,
        y: map_pos.y.floor() as i16,
        z: map_pos.z.floor() as i16,
    }
}
use crate::health::{CellUpdate, Health};
use std::collections::HashMap;
#[derive(Default)]
pub struct GridmapMain {
    pub grid_data: HashMap<Vec3Int, CellData>,
    pub entity_data: HashMap<Vec3Int, EntityGridData>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
}

pub struct EntityGridData {
    pub entity: Entity,
    pub entity_name: String,
}
use crate::examinable::RichName;
#[derive(Default)]
pub struct GridmapData {
    pub non_fov_blocking_cells_list: Vec<i64>,
    pub non_combat_obstacle_cells_list: Vec<i64>,
    pub non_laser_obstacle_cells_list: Vec<i64>,
    pub placeable_items_cells_list: Vec<i64>,
    pub ordered_main_names: Vec<String>,
    pub ordered_details1_names: Vec<String>,
    pub main_name_id_map: HashMap<String, i64>,
    pub main_id_name_map: HashMap<i64, String>,
    pub details1_name_id_map: HashMap<String, i64>,
    pub details1_id_name_map: HashMap<i64, String>,
    pub main_text_names: HashMap<i64, RichName>,
    pub details1_text_names: HashMap<i64, RichName>,
    pub main_text_examine_desc: HashMap<i64, String>,
    pub details1_text_examine_desc: HashMap<i64, String>,
    pub blackcell_id: i64,
    pub blackcell_blocking_id: i64,
    pub main_cell_properties: HashMap<i64, MainCellProperties>,
}
#[derive(Clone)]
pub struct MainCellProperties {
    pub id: i64,
    pub name: RichName,
    pub description: String,
    pub non_fov_blocker: bool,
    pub combat_obstacle: bool,
    pub placeable_item_surface: bool,
    pub laser_combat_obstacle: bool,
    pub collider: Collider,
    pub collider_position: Transform,
    pub constructable: bool,
    pub floor_cell: bool,
    pub atmospherics_blocker: bool,
    pub atmospherics_pushes_up: bool,
    pub direction_rotations: GridDirectionRotations,
    pub friction: f32,
    pub combine_rule: CoefficientCombineRule,
}
impl Default for MainCellProperties {
    fn default() -> Self {
        Self {
            id: 0,
            name: Default::default(),
            description: "".to_string(),
            non_fov_blocker: false,
            combat_obstacle: true,
            placeable_item_surface: false,
            laser_combat_obstacle: true,
            collider: Collider::cuboid(1., 1., 1.),
            collider_position: Transform::identity(),
            constructable: false,
            floor_cell: false,
            atmospherics_blocker: true,
            atmospherics_pushes_up: false,
            direction_rotations: GridDirectionRotations::default_wall_rotations(),
            friction: 0.,
            combine_rule: CoefficientCombineRule::Min,
        }
    }
}
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

pub fn cell_id_to_world(cell_id: Vec3Int) -> Vec3 {
    let mut world_position: Vec3 = Vec3::ZERO;

    world_position.x = (cell_id.x as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z as f32 * CELL_SIZE) + Y_CENTER_OFFSET;

    world_position
}

pub fn get_cell_name(ship_cell: &CellData, gridmap_data: &Res<GridmapData>) -> String {
    gridmap_data
        .main_text_names
        .get(&ship_cell.item)
        .unwrap()
        .get_a_name()
}
#[derive(Default)]
pub struct GridmapExamineMessages {
    pub messages: Vec<ExamineMapMessage>,
}
pub struct ExamineMapMessage {
    pub handle: u64,
    pub entity: Entity,
    pub gridmap_type: GridMapType,
    pub gridmap_cell_id: Vec3Int,
    pub message: String,
}
impl Default for ExamineMapMessage {
    fn default() -> Self {
        Self {
            handle: 0,
            entity: Entity::from_bits(0),
            gridmap_type: GridMapType::Main,
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
pub enum GridMapType {
    Main,
    Details1,
}

pub struct GridItemData {
    pub transform_offset: Transform,
    pub can_be_built_with_grid_item: Vec<String>,
}

pub struct RemoveCell {
    pub handle_option: Option<u64>,
    pub gridmap_type: GridMapType,
    pub id: Vec3Int,
    pub cell_data: CellData,
}
