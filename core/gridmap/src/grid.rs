use std::collections::HashMap;

use bevy::{
    prelude::{warn, Component, Entity, Handle, Res, Resource, Transform},
    scene::Scene,
};
use bevy_rapier3d::prelude::{CoefficientCombineRule, Collider};
use entity::{examine::RichName, health::Health};
use math::grid::Vec3Int;
use resources::grid::CellFace;
use serde::{Deserialize, Serialize};

/// Gridmap maximum limits as cube dimensions in chunks.
pub struct MapLimits {
    /// Full length of the cube as chunks.
    pub length: i16,
}

impl Default for MapLimits {
    fn default() -> Self {
        Self { length: 32 }
    }
}

/// Gridmap meta-data set.
#[derive(Clone)]

pub struct CellTileProperties {
    pub id: u16,
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
    /// Always available on client. Never available on server.
    pub mesh_option: Option<Handle<Scene>>,
}

impl Default for CellTileProperties {
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
            collider_position: Transform::IDENTITY,
            constructable: false,
            floor_cell: false,
            atmospherics_blocker: true,
            atmospherics_pushes_up: false,
            direction_rotations: GridDirectionRotations::default_wall_rotations(),
            friction: 0.,
            combine_rule: CoefficientCombineRule::Min,
            mesh_option: None,
        }
    }
}

pub fn get_cell_a_name(ship_cell: &CellItem, gridmap_data: &Res<Gridmap>) -> String {
    gridmap_data
        .main_text_names
        .get(&ship_cell.tile_type)
        .unwrap()
        .get_a_name()
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum Orientation {
    #[default]
    FrontFacing,
    BackFacing,
    RightFacing,
    LeftFacing,
}
#[derive(Clone, Default)]
pub struct GridCell {
    pub floor: Option<CellItem>,
    pub front_wall: Option<CellItem>,
    pub right_wall: Option<CellItem>,
}

impl GridCell {
    pub fn get_item(&self, strict_face: StrictCellFace) -> Option<CellItem> {
        match strict_face {
            StrictCellFace::FrontWall => self.front_wall.clone(),
            StrictCellFace::RightWall => self.right_wall.clone(),
            StrictCellFace::Floor => self.floor.clone(),
        }
    }
}

/// Data stored in a resource of a cell instead of each cell having their own entity with components.
#[derive(Clone, Default)]
pub struct CellItem {
    /// Id of tile type.
    pub tile_type: u16,
    /// Tile set id.
    pub group_id: Option<u32>,
    /// Entity belonging to item.
    pub entity: Option<Entity>,
    /// Shared group entity for tile.
    pub group_entity: Option<Entity>,
    /// Health of this tile.
    pub health: Health,
    /// Rotation.
    pub orientation: Orientation,
}

/// Maximum amount of available map chunks. 32 by 32 by 32 (cubic length of 1024 meters).
pub const GRID_CHUNK_AMOUNT: usize = 32768;
/// The amount of tiles a chunk stores. 32 by 32 by 32.
pub const GRID_CHUNK_TILES_AMOUNT: usize = 32768;
/// The length of the cubic chunk in tiles.
pub const CHUNK_CUBIC_LENGTH: i16 = 32;

/// A chunk of the gridmap.
#[derive(Clone)]
pub struct GridmapChunk {
    pub cells: Vec<Option<GridCell>>,
}

const DEFAULT_CELL_DATA: Option<GridCell> = None;

impl Default for GridmapChunk {
    fn default() -> Self {
        Self {
            cells: vec![DEFAULT_CELL_DATA; GRID_CHUNK_TILES_AMOUNT],
        }
    }
}

/// Stores the main gridmap layer data, huge map data resource. In favor of having each ordinary tile having its own entity with its own sets of components.
/// The hashmaps should probably be turned into arrays by converting Vec3Int into an index for performance reasons.
#[derive(Resource)]
pub struct Gridmap {
    pub grid: Vec<Option<GridmapChunk>>,
    pub updates: HashMap<Vec3Int, CellUpdate>,
    pub non_fov_blocking_cells_list: Vec<u16>,
    pub non_combat_obstacle_cells_list: Vec<u16>,
    pub non_laser_obstacle_cells_list: Vec<u16>,
    pub placeable_items_cells_list: Vec<u16>,
    pub ordered_main_names: Vec<String>,
    pub ordered_details1_names: Vec<String>,
    pub main_name_id_map: HashMap<String, u16>,
    pub main_id_name_map: HashMap<u16, String>,
    pub details1_name_id_map: HashMap<String, u16>,
    pub details1_id_name_map: HashMap<u16, String>,
    pub main_text_names: HashMap<u16, RichName>,
    pub details1_text_names: HashMap<u16, RichName>,
    pub main_text_examine_desc: HashMap<u16, String>,
    pub details1_text_examine_desc: HashMap<u16, String>,
    pub blackcell_id: u16,
    pub blackcell_blocking_id: u16,
    pub main_cell_properties: HashMap<u16, CellTileProperties>,
    pub map_length_limit: MapLimits,
}

const EMPTY_CHUNK: Option<GridmapChunk> = None;

impl Default for Gridmap {
    fn default() -> Self {
        Self {
            grid: vec![EMPTY_CHUNK; GRID_CHUNK_AMOUNT],
            updates: HashMap::default(),
            non_fov_blocking_cells_list: vec![],
            non_combat_obstacle_cells_list: vec![],
            non_laser_obstacle_cells_list: vec![],
            placeable_items_cells_list: vec![],
            ordered_main_names: vec![],
            ordered_details1_names: vec![],
            main_name_id_map: HashMap::default(),
            main_id_name_map: HashMap::default(),
            details1_name_id_map: HashMap::default(),
            details1_id_name_map: HashMap::default(),
            main_text_names: HashMap::default(),
            details1_text_names: HashMap::default(),
            main_text_examine_desc: HashMap::default(),
            details1_text_examine_desc: HashMap::default(),
            blackcell_id: 0,
            blackcell_blocking_id: 0,
            main_cell_properties: HashMap::default(),
            map_length_limit: MapLimits::default(),
        }
    }
}
/// Result for [get_indexes].
#[derive(Clone, Copy, Debug)]
pub struct CellIndexes {
    pub chunk: usize,
    pub cell: usize,
}

#[derive(Serialize, Deserialize, Default)]
pub enum StrictCellFace {
    #[default]
    FrontWall,
    RightWall,
    Floor,
}

pub struct StrictCell {
    pub face: StrictCellFace,
    pub id: Vec3Int,
}

/// Event to add a gridmap tile that can cover multiple cells.
pub struct SetCell {
    pub id: Vec3Int,
    pub orientation: Orientation,
    pub tile_id: u16,
    pub face: CellFace,
}

impl Gridmap {
    pub fn get_indexes(&self, id: Vec3Int) -> Option<CellIndexes> {
        let map_half_length = ((self.map_length_limit.length as f32 * CHUNK_CUBIC_LENGTH as f32)
            * 0.5)
            .floor() as i16;

        let x_id = id.x + map_half_length;
        let x_chunk_index = (x_id as f32 / CHUNK_CUBIC_LENGTH as f32).floor() as i16;

        let y_id = id.y + map_half_length;
        let y_chunk_index = (y_id as f32 / CHUNK_CUBIC_LENGTH as f32).floor() as i16;

        let z_id = id.z + map_half_length;
        let z_chunk_index = (z_id as f32 / CHUNK_CUBIC_LENGTH as f32).floor() as i16;

        let chunk_x_offset = x_chunk_index;
        let chunk_z_offset = z_chunk_index * self.map_length_limit.length;
        let chunk_y_offset =
            y_chunk_index * (self.map_length_limit.length * self.map_length_limit.length);

        let chunk_index = chunk_x_offset + chunk_z_offset + chunk_y_offset;

        let x_cell_id = x_id - (x_chunk_index * CHUNK_CUBIC_LENGTH);
        let y_cell_id = y_id - (y_chunk_index * CHUNK_CUBIC_LENGTH);
        let z_cell_id = z_id - (z_chunk_index * CHUNK_CUBIC_LENGTH);

        let x_offset = x_cell_id;
        let z_offset = z_cell_id * CHUNK_CUBIC_LENGTH;
        let y_offset = y_cell_id * (CHUNK_CUBIC_LENGTH * CHUNK_CUBIC_LENGTH);

        let cell_index = x_offset + z_offset + y_offset;
        Some(CellIndexes {
            chunk: chunk_index as usize,
            cell: cell_index as usize,
        })
    }
    pub fn get_id(&self, indexes: CellIndexes) -> Option<Vec3Int> {
        let chunk_y_id = (indexes.chunk as f32
            / (self.map_length_limit.length * self.map_length_limit.length) as f32)
            .floor() as i16;

        let remainder_xz = indexes.chunk as i16
            - (chunk_y_id * (self.map_length_limit.length * self.map_length_limit.length));

        let chunk_z_id = (remainder_xz as f32 / self.map_length_limit.length as f32).floor() as i16;

        let chunk_x_id = remainder_xz - (chunk_z_id * self.map_length_limit.length);

        let cell_y_id =
            (indexes.cell as f32 / (CHUNK_CUBIC_LENGTH * CHUNK_CUBIC_LENGTH) as f32).floor() as i16;

        let remainder_xz =
            indexes.cell as i16 - (cell_y_id * (CHUNK_CUBIC_LENGTH * CHUNK_CUBIC_LENGTH));

        let cell_z_id = (remainder_xz as f32 / CHUNK_CUBIC_LENGTH as f32).floor() as i16;

        let cell_x_id = remainder_xz - (cell_z_id * CHUNK_CUBIC_LENGTH);

        let map_half_length = ((self.map_length_limit.length as f32 * CHUNK_CUBIC_LENGTH as f32)
            * 0.5)
            .floor() as i16;

        let id = Vec3Int {
            x: (chunk_x_id * CHUNK_CUBIC_LENGTH + cell_x_id) - map_half_length,
            y: (chunk_y_id * CHUNK_CUBIC_LENGTH + cell_y_id) - map_half_length,
            z: (chunk_z_id * CHUNK_CUBIC_LENGTH + cell_z_id) - map_half_length,
        };

        Some(id)
    }
    pub fn get_strict_cell(&self, id: Vec3Int, face: CellFace) -> StrictCell {
        let mut adjusted_id = id.clone();
        let adjusted_face;

        match face {
            CellFace::BackWall => {
                adjusted_id.z -= 1;
                adjusted_face = StrictCellFace::FrontWall;
            }
            CellFace::LeftWall => {
                adjusted_id.x -= 1;
                adjusted_face = StrictCellFace::RightWall;
            }
            CellFace::Ceiling => {
                adjusted_id.y += 1;
                adjusted_face = StrictCellFace::Floor;
            }
            CellFace::FrontWall => {
                adjusted_face = StrictCellFace::FrontWall;
            }
            CellFace::RightWall => {
                adjusted_face = StrictCellFace::RightWall;
            }
            CellFace::Floor => {
                adjusted_face = StrictCellFace::Floor;
            }
        }
        StrictCell {
            face: adjusted_face,
            id: adjusted_id,
        }
    }
    pub fn get_cell(&self, id: Vec3Int, face: CellFace) -> Option<CellItem> {
        let strict = self.get_strict_cell(id, face);

        let indexes;
        match self.get_indexes(strict.id) {
            Some(i) => {
                indexes = i;
            }
            None => {
                warn!("Couldnt get index.");
                return None;
            }
        }

        match self.grid.get(indexes.chunk) {
            Some(chunk_option) => match chunk_option {
                Some(chunk) => match chunk.cells.get(indexes.cell) {
                    Some(cell_data_option) => match cell_data_option {
                        Some(items) => items.get_item(strict.face),
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}

/// For entities that are also registered in the gridmap. (entity tiles)

pub struct EntityGridData {
    pub entity: Entity,
    pub entity_type: String,
}
/// Directional rotations alongside their "orientation" value used for Godot gridmaps.
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

/// Remove gridmap cell event.

pub struct RemoveCell {
    pub handle_option: Option<u64>,
    pub id: Vec3Int,
    pub face: CellFace,
}

/// A pending cell update like a cell construction.

pub struct CellUpdate {
    pub entities_received: Vec<Entity>,
    pub cell_data: CellItem,
}

/// Component that represents a cell.
#[derive(Component)]

pub struct Cell {
    pub id: Vec3Int,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            id: Vec3Int { x: 0, y: 0, z: 0 },
        }
    }
}
