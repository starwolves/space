use std::collections::HashMap;
use serde::{Deserialize};

use super::precalculated_fov_data::Vec3Int;

pub struct GridmapMain {
    pub data : HashMap<Vec3Int, CellData>
}

#[derive(Deserialize)]
pub struct CellDataWID {
    pub id: String,
    pub item: i64,
    pub orientation: i64
}

pub struct CellData {
    pub item: i64,
    pub orientation: i64,
}
