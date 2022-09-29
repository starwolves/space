use std::collections::HashMap;

use api::gridmap::{CellData, GridDirectionRotations};
use bevy::prelude::{Res, Transform};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider};
use examinable::examine::RichName;

/// Gridmap meta-data resource.
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
/// Gridmap meta-data set.
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

pub fn get_cell_a_name(ship_cell: &CellData, gridmap_data: &Res<GridmapData>) -> String {
    gridmap_data
        .main_text_names
        .get(&ship_cell.item)
        .unwrap()
        .get_a_name()
}
