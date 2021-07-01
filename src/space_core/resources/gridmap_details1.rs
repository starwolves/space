use std::collections::HashMap;

use super::{gridmap_main::CellData, precalculated_fov_data::Vec3Int};

pub struct GridmapDetails1 {
    pub data : HashMap<Vec3Int, CellData>
}
