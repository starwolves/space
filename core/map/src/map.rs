use std::collections::HashMap;

use api::{data::Vec2Int, gridmap::FOV_MAP_WIDTH};

use bevy::{
    math::Vec2,
    prelude::{Component, Entity},
};

/// Mini-map overlay tile color.
#[derive(Clone, PartialEq)]
pub enum OverlayTile {
    Green,
    Yellow,
    Orange,
    Red,
}

/// Get overlay tile priority.
pub fn get_overlay_tile_priority(tile: &OverlayTile) -> u8 {
    match tile {
        OverlayTile::Green => 0,
        OverlayTile::Yellow => 1,
        OverlayTile::Orange => 2,
        OverlayTile::Red => 3,
    }
}

/// Get overlay tile item.
pub fn get_overlay_tile_item(tile: &OverlayTile) -> i16 {
    match tile {
        OverlayTile::Green => 0,
        OverlayTile::Yellow => 3,
        OverlayTile::Orange => 1,
        OverlayTile::Red => 2,
    }
}
/// Cache atmospherics mini-map overlay.
#[derive(Clone, Default)]
pub struct AtmosphericsCache {
    pub tile_color: Option<OverlayTile>,
}

pub const GREEN_MAP_TILE_ENTRANCE: i16 = 3;
pub const GREEN_MAP_TILE_COUNTER: i16 = 4;

/// Data regarding an entity that has a mini-map.
pub struct MapHolderData {
    /// Start increment of the next batch.
    pub batch_i: usize,
    /// Previous batch.
    pub cache: Vec<AtmosphericsCache>,
    /// Previous camera position.
    pub prev_camera_cell_id: Vec2Int,
    /// Previous camera angle.
    pub prev_camera_view_range: usize,
    /// Should the cache be reset?
    pub reset_cache: bool,
    /// Currently displayed data text to client.
    pub hovering_data: String,
}

/// All mini-maps of entities.
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

/// Mini-map of a player.
#[derive(Component)]
pub struct Map {
    /// Currently active display overlay.
    pub display_mode: Option<String>,
    /// Available display overlays.
    pub available_display_modes: Vec<(String, String)>,
    /// Map camera view distance.
    pub view_range: usize,
    /// Map camera position.
    pub camera_position: Vec2,
    /// Selected mouse cell.
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
