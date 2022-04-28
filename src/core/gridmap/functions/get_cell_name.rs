use bevy_ecs::system::Res;

use crate::core::gridmap::resources::{CellData, GridmapData};

pub fn get_cell_name(ship_cell: &CellData, gridmap_data: &Res<GridmapData>) -> String {
    gridmap_data
        .main_text_names
        .get(&ship_cell.item)
        .unwrap()
        .get_a_name()
}
