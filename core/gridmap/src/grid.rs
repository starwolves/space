use std::collections::HashMap;

use bevy::prelude::{Entity, Res, Transform, Vec3};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider};
use entity::{examine::RichName, health::Health};
use math::grid::{Vec3Int, CELL_SIZE};
use networking::server::GridMapLayer;

/// Gridmap meta-data resource.
#[derive(Default)]
#[cfg(feature = "server")]
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
/// Gridmap meta-data set.
#[derive(Clone)]
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
pub fn get_cell_a_name(ship_cell: &CellData, gridmap_data: &Res<GridmapData>) -> String {
    gridmap_data
        .main_text_names
        .get(&ship_cell.item)
        .unwrap()
        .get_a_name()
}

/// Data stored in a resource of a cell instead of each cell having their own entity with components.
#[derive(Clone, Default)]
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub struct GridmapDetails1 {
    pub grid_data: HashMap<Vec3Int, CellData>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
}

/// Stores the main gridmap layer data, huge map data resource. In favor of having each ordinary tile having its own entity with its own sets of components.
#[derive(Default)]
#[cfg(feature = "server")]
pub struct GridmapMain {
    pub grid_data: HashMap<Vec3Int, CellData>,
    pub entity_data: HashMap<Vec3Int, EntityGridData>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
}

/// For entities that are also registered in the gridmap. (entity tiles)
#[cfg(feature = "server")]
pub struct EntityGridData {
    pub entity: Entity,
    pub entity_name: String,
}
/// Directional rotations alongside their "orientation" value used for Godot gridmaps.
#[derive(Clone)]
#[cfg(feature = "server")]
pub struct GridDirectionRotations {
    pub data: HashMap<AdjacentTileDirection, u8>,
}

#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub enum AdjacentTileDirection {
    Up,
    Down,
    Left,
    Right,
}
#[cfg(feature = "server")]
const Y_CENTER_OFFSET: f32 = 1.;

/// From tile id to world position.
#[cfg(feature = "server")]
pub fn cell_id_to_world(cell_id: Vec3Int) -> Vec3 {
    let mut world_position: Vec3 = Vec3::ZERO;

    world_position.x = (cell_id.x as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y as f32 * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z as f32 * CELL_SIZE) + Y_CENTER_OFFSET;

    world_position
}

/// Remove gridmap cell event.
#[cfg(feature = "server")]
pub struct RemoveCell {
    pub handle_option: Option<u64>,
    pub gridmap_type: GridMapLayer,
    pub id: Vec3Int,
    pub cell_data: CellData,
}

/// A pending cell update like a cell construction.
#[cfg(feature = "server")]
pub struct CellUpdate {
    pub entities_received: Vec<Entity>,
    pub cell_data: CellData,
}
