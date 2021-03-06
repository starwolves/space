#[derive(Clone, PartialEq)]
pub enum OverlayTile {
    Green,
    Yellow,
    Orange,
    Red,
}

pub fn get_overlay_tile_priority(tile: &OverlayTile) -> u8 {
    match tile {
        OverlayTile::Green => 0,
        OverlayTile::Yellow => 1,
        OverlayTile::Orange => 2,
        OverlayTile::Red => 3,
    }
}

pub fn get_overlay_tile_item(tile: &OverlayTile) -> i16 {
    match tile {
        OverlayTile::Green => 0,
        OverlayTile::Yellow => 3,
        OverlayTile::Orange => 1,
        OverlayTile::Red => 2,
    }
}
#[derive(Clone, Default)]
pub struct AtmosphericsCache {
    pub tile_color: Option<OverlayTile>,
}

use std::collections::HashMap;

pub const GREEN_MAP_TILE_ENTRANCE: i16 = 3;
pub const GREEN_MAP_TILE_COUNTER: i16 = 4;
use api::{data::Vec2Int, gridmap::FOV_MAP_WIDTH};

use bevy::{
    math::Vec2,
    prelude::{Component, Entity},
};

pub struct MapHolderData {
    pub batch_i: usize,
    pub cache: Vec<AtmosphericsCache>,
    pub prev_camera_cell_id: Vec2Int,
    pub prev_camera_view_range: usize,
    pub reset_cache: bool,
    pub hovering_data: String,
}
#[derive(Default)]
pub struct MapHolders {
    pub holders: HashMap<Entity, MapHolderData>,
}

impl Default for MapHolderData {
    fn default() -> Self {
        Self {
            batch_i: 0,
            cache: vec![AtmosphericsCache::default(); FOV_MAP_WIDTH * FOV_MAP_WIDTH],
            prev_camera_cell_id: Vec2Int::default(),
            prev_camera_view_range: 20,
            reset_cache: true,
            hovering_data: "".to_string(),
        }
    }
}

#[derive(Component)]
pub struct Map {
    pub display_mode: Option<String>,
    pub available_display_modes: Vec<(String, String)>,
    pub view_range: usize,
    pub camera_position: Vec2,
    pub passed_mouse_cell: Option<(i16, i16)>,
}
impl Default for Map {
    fn default() -> Self {
        Self {
            display_mode: None,
            available_display_modes: vec![("Standard".to_string(), "standard".to_string())],
            view_range: 20,
            camera_position: Vec2::default(),
            passed_mouse_cell: None,
        }
    }
}
