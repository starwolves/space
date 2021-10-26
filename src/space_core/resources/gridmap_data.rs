use std::collections::HashMap;

use bevy::prelude::{FromWorld, World};

use crate::space_core::{components::examinable::RichName, systems::general::startup_init_gridmap_cells::MainCellProperties};

pub struct GridmapData {
    pub non_fov_blocking_cells_list: Vec<i64>,
    pub non_combat_obstacle_cells_list: Vec<i64>,
    pub non_laser_obstacle_cells_list: Vec<i64>,
    pub placeable_items_cells_list: Vec<i64>,
    pub ordered_main_names : Vec<String>,
    pub ordered_details1_names: Vec<String>,
    pub main_name_id_map : HashMap<String, i64>,
    pub main_id_name_map : HashMap<i64, String>,
    pub details1_name_id_map: HashMap<String, i64>,
    pub details1_id_name_map: HashMap<i64,String>,
    pub main_text_names : HashMap<i64, RichName>,
    pub details1_text_names  : HashMap<i64, RichName>,
    pub main_text_examine_desc : HashMap<i64, String>,
    pub details1_text_examine_desc : HashMap<i64, String>,
    pub blackcell_id : i64,
    pub blackcell_blocking_id : i64,
    pub main_cell_properties : HashMap<i64, MainCellProperties>,
}

impl FromWorld for GridmapData {
    fn from_world(_world: &mut World) -> Self {
        GridmapData {
            non_fov_blocking_cells_list : vec![],
            non_combat_obstacle_cells_list : vec![],
            non_laser_obstacle_cells_list : vec![],
            placeable_items_cells_list : vec![],
            ordered_main_names : vec![],
            ordered_details1_names : vec![],
            main_name_id_map : HashMap::new(),
            main_id_name_map : HashMap::new(),
            details1_name_id_map : HashMap::new(),
            details1_id_name_map : HashMap::new(),
            main_text_names : HashMap::new(),
            details1_text_names  : HashMap::new(),
            main_text_examine_desc : HashMap::new(),
            details1_text_examine_desc : HashMap::new(),
            blackcell_id : 0,
            blackcell_blocking_id : 0,
            main_cell_properties: HashMap::new(),
        }
    }
}
