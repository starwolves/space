
use bevy::prelude::Res;

use crate::space_core::resources::{gridmap_data::GridmapData, gridmap_main::CellData};


pub fn get_cell_name(
    ship_cell : &CellData,
    gridmap_data : &Res<GridmapData>,
) -> String {

    gridmap_data.main_text_names.get(&ship_cell.item).unwrap().get_a_name()

}
