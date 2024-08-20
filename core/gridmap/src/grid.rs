use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, f32::consts::PI, ops::Deref};

use bevy::log::{info, warn};
use bevy::pbr::PointLight;
use bevy::{
    gltf::GltfMesh,
    prelude::{
        Commands, Component, Entity, Event, EventWriter, Handle, Mat3, Quat, Query, Res, Resource,
        StandardMaterial, SystemSet, Transform, Vec3, Without,
    },
};

use bevy_renet::renet::ClientId;
use bevy_xpbd_3d::components::{CoefficientCombine, CollisionLayers, Friction, RigidBody};
use bevy_xpbd_3d::prelude::Collider;
use entity::{despawn::DespawnEntity, examine::RichName, health::Health};
use networking::{
    client::IncomingReliableServerMessage,
    server::{ConnectedPlayer, OutgoingReliableServerMessage},
};
use physics::physics::{get_bit_masks, ColliderGroup};
use physics::rigid_body::STANDARD_BODY_FRICTION;
use resources::grid::GridmapCollider;
use resources::grid::{CellFace, Tile};
use resources::player::SoftPlayer;
use resources::{
    grid::TargetCell,
    math::{cell_id_to_world, Vec3Int},
};
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
#[derive(Clone, Debug)]
pub enum CellType {
    Wall,
    WallDetail,
    Floor,
    Center,
}

impl CellType {
    pub fn default_face(&self) -> CellFace {
        match self {
            CellType::Wall => CellFace::FrontWall,
            CellType::WallDetail => CellFace::FrontWall,
            CellType::Floor => CellFace::Floor,
            CellType::Center => CellFace::Center,
        }
    }
}

/// Gridmap meta-data set.
#[derive(Clone, Debug)]

pub struct TileGroup {
    pub name_id: GroupTypeName,
    pub map: HashMap<Vec3Int, FullCell>,
}
#[derive(Clone, Debug)]
pub struct TileLight {
    pub light: PointLight,
    pub local_offset: Vec3,
}

/// Gridmap meta-data set.
#[derive(Clone, Debug)]

pub struct TileProperties {
    pub name_id: CellTypeName,
    pub name: RichName,
    pub description: String,
    pub non_fov_blocker: bool,
    pub combat_obstacle: bool,
    pub placeable_item_surface: bool,
    pub laser_combat_obstacle: bool,
    pub collider: Collider,
    pub collider_position: Transform,
    pub constructable: bool,
    pub vertical_rotation: bool,
    pub atmospherics_blocker: bool,
    pub atmospherics_pushes_up: bool,
    /// Always available on client. Never available on server.
    pub mesh_option: Option<Handle<GltfMesh>>,
    pub material_option: Option<Handle<StandardMaterial>>,
    pub cell_type: CellType,
    pub x_rotations: Vec<u8>,
    pub y_rotations: Vec<u8>,
    pub is_detail: bool,
    pub is_light: Option<TileLight>,
}

impl Default for TileProperties {
    fn default() -> Self {
        Self {
            name_id: CellTypeName("".to_string()),
            name: Default::default(),
            description: "".to_string(),
            non_fov_blocker: false,
            combat_obstacle: true,
            placeable_item_surface: false,
            laser_combat_obstacle: true,
            collider: Collider::cuboid(1., 1., 1.),
            collider_position: Transform::IDENTITY,
            constructable: false,
            atmospherics_blocker: true,
            atmospherics_pushes_up: false,
            mesh_option: None,
            material_option: None,
            cell_type: CellType::Wall,
            x_rotations: vec![],
            y_rotations: vec![],
            vertical_rotation: false,
            is_detail: false,
            is_light: None,
        }
    }
}

pub fn get_cell_a_name(ship_cell: &CellItem, gridmap_data: &Res<Gridmap>) -> String {
    gridmap_data
        .tile_properties
        .get(&ship_cell.tile_type)
        .unwrap()
        .name
        .get_a_name()
}
#[derive(Clone, Default, Debug)]
pub struct GridCell {
    pub floor: Option<CellItem>,
    pub front_wall: Option<CellItem>,
    pub right_wall: Option<CellItem>,
    pub center: Option<CellItem>,
}

impl GridCell {
    pub fn get_item_from_face(&self, strict_face: StrictCellFace) -> Option<CellItem> {
        match strict_face {
            StrictCellFace::FrontWall => self.front_wall.clone(),
            StrictCellFace::RightWall => self.right_wall.clone(),
            StrictCellFace::Floor => self.floor.clone(),
            StrictCellFace::Center => self.center.clone(),
        }
    }
    pub fn get_items(&self) -> Vec<(CellItem, CellFace)> {
        let mut items = vec![];
        match &self.floor {
            Some(i) => {
                items.push((i.clone(), CellFace::Floor));
            }
            None => {}
        }
        match &self.front_wall {
            Some(i) => {
                items.push((i.clone(), CellFace::FrontWall));
            }
            None => {}
        }
        match &self.right_wall {
            Some(i) => {
                items.push((i.clone(), CellFace::RightWall));
            }
            None => {}
        }
        match &self.center {
            Some(i) => {
                items.push((i.clone(), CellFace::Center));
            }
            None => {}
        }
        items
    }
    pub fn is_empty(&self) -> bool {
        self.floor.is_none()
            && self.front_wall.is_none()
            && self.right_wall.is_none()
            && self.center.is_none()
    }
}

/// Data stored in a resource of a cell instead of each cell having their own entity with components.
#[derive(Clone, Default, Debug)]
pub struct CellItem {
    /// Id of tile type.
    pub tile_type: CellTypeId,
    /// Instance id of gridmap group.
    pub group_id_option: Option<u32>,
    /// Entity belonging to cell item.
    pub entity: Option<Entity>,
    /// Health of this tile.
    pub health: Health,
    /// Rotation. Range of 0 - 24. See [OrthogonalBases].
    pub orientation: u8,
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

impl GridmapChunk {
    fn is_empty(&self) -> bool {
        let mut empty = true;

        for cell in self.cells.iter() {
            if cell.is_some() {
                empty = false;
                break;
            }
        }

        empty
    }
}

#[derive(Clone)]
pub struct AddedUpdate {
    pub cell: GridmapUpdate,
    pub players_received: Vec<ClientId>,
}
#[derive(Clone)]
pub enum GridmapUpdate {
    Added(NewCell),
    Removed,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CellIds {
    CellType(CellTypeId),
    GroupType(GroupTypeId),
}

impl CellIds {
    pub fn to_string(&self) -> String {
        match self {
            CellIds::CellType(id) => id.to_string(),
            CellIds::GroupType(id) => id.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Default)]
/// Identifier used for exports and imports.
pub struct CellTypeName(pub String);
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Default, Copy)]
/// Each cell type name has a u16 id for efficiency.
pub struct CellTypeId(pub u16);
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct GroupTypeName(pub String);
#[derive(Serialize, Deserialize, Debug, Clone, Eq, Hash, PartialEq, Default, Copy)]
pub struct GroupTypeId(pub u16);

impl Deref for CellTypeName {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for CellTypeId {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for GroupTypeName {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for GroupTypeId {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
pub struct LayerTargetCell {
    pub target: TargetCell,
    pub is_detail: bool,
}

/// Stores the main gridmap layer data, huge map data resource. In favor of having each ordinary tile having its own entity with its own sets of components.
#[derive(Resource)]
pub struct Gridmap {
    pub main_grid: Vec<Option<GridmapChunk>>,
    pub details_grid: Vec<Option<GridmapChunk>>,
    pub updates: HashMap<LayerTargetCell, AddedUpdate>,
    pub ordered_names: Vec<CellTypeName>,
    pub group_id_map: HashMap<GroupTypeName, GroupTypeId>,
    pub id_group_map: HashMap<GroupTypeId, GroupTypeName>,
    pub tile_properties: HashMap<CellTypeId, TileProperties>,
    pub map_length_limit: MapLimits,
    pub groups: HashMap<GroupTypeId, HashMap<Vec3Int, FullCell>>,
    pub group_instance_incremental: u32,
    pub tile_type_incremental: u16,
    pub group_type_incremental: u16,
    pub name_id_map: HashMap<CellTypeName, CellTypeId>,
    pub id_name_map: HashMap<CellTypeId, CellTypeName>,
    // Colliders for chunks.
    pub colliders: HashMap<usize, Entity>,
}
const EMPTY_CHUNK: Option<GridmapChunk> = None;

impl Default for Gridmap {
    fn default() -> Self {
        Self {
            main_grid: vec![EMPTY_CHUNK; GRID_CHUNK_AMOUNT],
            details_grid: vec![EMPTY_CHUNK; GRID_CHUNK_AMOUNT],
            updates: HashMap::default(),
            ordered_names: vec![],
            tile_properties: HashMap::default(),
            map_length_limit: MapLimits::default(),
            groups: HashMap::default(),
            group_id_map: HashMap::default(),
            id_group_map: HashMap::default(),
            group_instance_incremental: 0,
            tile_type_incremental: 0,
            group_type_incremental: 0,
            name_id_map: HashMap::default(),
            id_name_map: HashMap::default(),
            colliders: HashMap::default(),
        }
    }
}
/// Result for [get_indexes].
#[derive(Clone, Copy, Debug)]
pub struct CellIndexes {
    pub chunk: usize,
    pub cell: usize,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum StrictCellFace {
    #[default]
    FrontWall,
    RightWall,
    Floor,
    Center,
}

pub struct StrictCell {
    pub face: StrictCellFace,
    pub id: Vec3Int,
}

/// Event to add a gridmap tile that can cover multiple cells.
pub struct SetCell {
    pub id: Vec3Int,
    pub orientation: u8,
    pub tile_id: u16,
    pub face: CellFace,
}
pub const EXPORT_TEST_MAP: bool = false;
pub(crate) fn export_debug_map() {
    if !EXPORT_TEST_MAP {
        return;
    }
    let mut data: Vec<CellDataExport> = vec![];

    let floor_length = 20;

    let mut x = -floor_length / 2;
    let mut y = -floor_length / 2;
    for _ in 0..(floor_length * floor_length) - 1 {
        data.push(CellDataExport {
            id: Vec3Int { x: x, y: 0, z: y },
            orientation: 0,
            face: CellFace::Floor,
            item: ItemExport::Cell(CellTypeName("generic_floor".to_string())),
            is_detail: false,
        });
        x += 1;
        if x > floor_length / 2 {
            x = -floor_length / 2;
            y += 1;
        }
    }

    let dats = bincode::serialize(&data).unwrap();
    let path = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("test_map.bin");
    let mut file = File::create(path.clone()).unwrap();
    file.write_all(&dats).unwrap();
    info!("Exported debug map!");
}

impl Gridmap {
    pub fn export_binary(&self) -> Vec<u8> {
        let mut data = vec![];
        let mut chunk_i = 0;
        for chunk_option in &self.main_grid {
            let details_chunk_option = &self.details_grid[chunk_i];

            match details_chunk_option {
                Some(chunk) => {
                    let mut cell_i = 0;

                    for cell_option in chunk.cells.iter() {
                        match cell_option {
                            Some(cell) => {
                                for (item, face) in cell.get_items() {
                                    let cell_item_id;

                                    match self.id_name_map.get(&item.tile_type) {
                                        Some(x) => {
                                            cell_item_id = x.clone();
                                        }
                                        None => {
                                            warn!("Couldnt find item {:?}", item.tile_type);
                                            continue;
                                        }
                                    };
                                    let cell_item;

                                    match item.group_id_option {
                                        Some(group_id) => {
                                            let name;
                                            match self.id_name_map.get(&item.tile_type) {
                                                Some(n) => {
                                                    name = n;
                                                }
                                                None => {
                                                    warn!("couldnt find name");
                                                    continue;
                                                }
                                            }
                                            cell_item = ItemExport::Group(GroupItem {
                                                name: GroupTypeName(cell_item_id.to_string()),
                                                group_id: group_id,
                                                cell: name.clone(),
                                            });
                                        }
                                        None => {
                                            cell_item = ItemExport::Cell(CellTypeName(
                                                cell_item_id.to_string(),
                                            ));
                                        }
                                    }
                                    data.push(CellDataExport {
                                        id: self
                                            .get_id(CellIndexes {
                                                chunk: chunk_i,
                                                cell: cell_i,
                                            })
                                            .unwrap(),
                                        item: cell_item,
                                        orientation: item.orientation,
                                        face: face,
                                        is_detail: true,
                                    });
                                }
                            }
                            None => {}
                        }
                        cell_i += 1;
                    }
                }
                None => {}
            }

            match chunk_option {
                Some(chunk) => {
                    let mut cell_i = 0;

                    for cell_option in chunk.cells.iter() {
                        match cell_option {
                            Some(cell) => {
                                for (item, face) in cell.get_items() {
                                    let cell_item_id;

                                    match self.id_name_map.get(&item.tile_type) {
                                        Some(x) => {
                                            cell_item_id = x.clone();
                                        }
                                        None => {
                                            warn!("Couldnt find item {:?}", item.tile_type);
                                            continue;
                                        }
                                    };
                                    let cell_item;

                                    match item.group_id_option {
                                        Some(group_id) => {
                                            let name;
                                            match self.id_name_map.get(&item.tile_type) {
                                                Some(n) => {
                                                    name = n;
                                                }
                                                None => {
                                                    warn!("couldnt find name");
                                                    continue;
                                                }
                                            }
                                            cell_item = ItemExport::Group(GroupItem {
                                                name: GroupTypeName(cell_item_id.to_string()),
                                                group_id: group_id,
                                                cell: name.clone(),
                                            });
                                        }
                                        None => {
                                            cell_item = ItemExport::Cell(CellTypeName(
                                                cell_item_id.to_string(),
                                            ));
                                        }
                                    }

                                    data.push(CellDataExport {
                                        id: self
                                            .get_id(CellIndexes {
                                                chunk: chunk_i,
                                                cell: cell_i,
                                            })
                                            .unwrap(),
                                        item: cell_item,
                                        orientation: item.orientation,
                                        face: face,
                                        is_detail: false,
                                    });
                                }
                            }
                            None => {}
                        }
                        cell_i += 1;
                    }
                }
                None => {}
            }
            chunk_i += 1;
        }
        bincode::serialize(&data).unwrap()
    }
    pub fn get_indexes(&self, id: Vec3Int) -> CellIndexes {
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
        CellIndexes {
            chunk: chunk_index as usize,
            cell: cell_index as usize,
        }
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
    pub fn get_strict_cell(&self, cell: TargetCell) -> StrictCell {
        let mut adjusted_id = cell.id.clone();
        let adjusted_face;

        match cell.face {
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
            CellFace::Center => {
                adjusted_face = StrictCellFace::Center;
            }
        }
        StrictCell {
            face: adjusted_face,
            id: adjusted_id,
        }
    }
    pub fn get_cell(&self, cell: TargetCell) -> Option<CellItem> {
        let strict = self.get_strict_cell(cell);

        let indexes = self.get_indexes(strict.id);

        match self.main_grid.get(indexes.chunk) {
            Some(chunk_option) => match chunk_option {
                Some(chunk) => match chunk.cells.get(indexes.cell) {
                    Some(cell_data_option) => match cell_data_option {
                        Some(items) => items.get_item_from_face(strict.face),
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }

    pub fn get_cell_transform(&self, cell: TargetCell, orientation: u8) -> Transform {
        let strict = self.get_strict_cell(cell);

        let mut transform = Transform::from_translation(cell_id_to_world(strict.id));
        match strict.face {
            crate::grid::StrictCellFace::FrontWall => {
                transform.translation.z += 0.5;
                transform.translation.y += 0.5;
                transform.rotation = Quat::from_rotation_y(1. * PI);
            }
            crate::grid::StrictCellFace::RightWall => {
                transform.translation.x += 0.5;
                transform.translation.y += 0.5;
                transform.rotation = Quat::from_rotation_y(0.5 * PI);
            }
            crate::grid::StrictCellFace::Floor => {}
            StrictCellFace::Center => {
                transform.translation.y += 0.5;
            }
        }
        transform.rotation *= OrthogonalBases::default().bases[orientation as usize];
        transform
    }
    pub fn get_chunk_collider_data(&self, chunk_id: usize) -> Vec<(Vec3, Quat, Collider)> {
        match self.main_grid.get(chunk_id) {
            Some(chunk_data_option) => match chunk_data_option {
                Some(chunk_data) => {
                    let mut data = vec![];
                    let mut cell_id = 0;
                    for cell_option in chunk_data.cells.iter() {
                        match cell_option {
                            Some(cell_data) => {
                                for (cell_item, cell_face) in cell_data.get_items() {
                                    match self.tile_properties.get(&cell_item.tile_type) {
                                        Some(properties) => {
                                            let mut world_position = self.get_cell_transform(
                                                TargetCell {
                                                    id: self
                                                        .get_id(CellIndexes {
                                                            chunk: chunk_id,
                                                            cell: cell_id,
                                                        })
                                                        .unwrap(),
                                                    face: cell_face,
                                                },
                                                cell_item.orientation,
                                            );
                                            let relative_collider_position =
                                                OrthogonalBases::default().bases
                                                    [cell_item.orientation as usize]
                                                    .mul_vec3(
                                                        properties.collider_position.translation,
                                                    );

                                            world_position.translation +=
                                                relative_collider_position;
                                            world_position.rotation *=
                                                properties.collider_position.rotation;
                                            data.push((
                                                world_position.translation,
                                                world_position.rotation,
                                                properties.collider.clone(),
                                            ));
                                        }
                                        None => {
                                            warn!("No tile_properties found for cell type.");
                                        }
                                    }
                                }
                            }
                            None => {}
                        }
                        cell_id += 1;
                    }
                    data
                }
                None => vec![],
            },
            None => {
                warn!("Chunk doesnt exist.");
                vec![]
            }
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
#[derive(Event)]
pub struct RemoveTile {
    pub cell: LayerTargetCell,
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum EditTileSet {
    Add,
    Remove,
}

/// Event to add a gridmap tile.
#[derive(Event, Clone)]
pub struct AddTile {
    pub id: Vec3Int,
    /// Id of tile type.
    pub tile_type: CellTypeId,
    /// Rotation.
    pub orientation: u8,
    pub face: CellFace,
    pub group_instance_id_option: Option<u32>,
    pub entity: Entity,
    pub default_map_spawn: bool,
    pub is_detail: bool,
}

#[derive(Clone, Debug)]
pub struct FullCell {
    pub face: CellFace,
    pub orientation: u8,
    pub tile_type: CellTypeId,
    pub entity_option: Option<Entity>,
}

/// Event to add a group of gridmap tiles.
#[derive(Default, Event)]
pub struct AddGroup {
    pub id: Vec3Int,
    /// Group id.
    pub group_id: GroupTypeId,
    /// Rotation.
    pub orientation: u8,
    pub face: CellFace,
    pub default_map_spawn: bool,
}

use bevy::prelude::{EventReader, ResMut, TransformBundle};
use entity::health::{HealthContainer, HealthFlag, StructureHealth};

use crate::{
    init::{CellDataExport, GroupItem, ItemExport},
    net::{GridmapServerMessage, NewCell},
};
pub(crate) fn remove_tile(
    mut events: EventReader<RemoveTile>,
    mut gridmap: ResMut<Gridmap>,
    mut despawn: EventWriter<DespawnEntity>,
    mut gridmap_collider_query: Query<(&GridmapCollider, &mut Collider)>,
) {
    let mut changed_chunks = vec![];
    for event in events.read() {
        let chunk_id = gridmap.get_indexes(event.cell.target.id).chunk;
        if !changed_chunks.contains(&chunk_id) {
            changed_chunks.push(chunk_id);
        }
        let strict_cell = gridmap.get_strict_cell(event.cell.target.clone());
        let indexes = gridmap.get_indexes(strict_cell.id);
        let grid;
        if !event.cell.is_detail {
            grid = &mut gridmap.main_grid;
        } else {
            grid = &mut gridmap.details_grid;
        }
        match grid.get_mut(indexes.chunk) {
            Some(grid_chunk_option) => {
                let mut clear_chunk = false;
                match grid_chunk_option {
                    Some(grid_chunk) => {
                        match grid_chunk.cells.get_mut(indexes.cell) {
                            Some(cell_option) => {
                                let mut cell_empty = false;
                                match cell_option {
                                    Some(cell) => {
                                        let mut old_cell_entity = None;
                                        match strict_cell.face {
                                            StrictCellFace::FrontWall => {
                                                match &cell.front_wall {
                                                    Some(wall) => {
                                                        old_cell_entity = wall.entity;
                                                    }
                                                    None => {}
                                                }

                                                cell.front_wall = None;
                                            }
                                            StrictCellFace::RightWall => {
                                                match &cell.right_wall {
                                                    Some(wall) => {
                                                        old_cell_entity = wall.entity;
                                                    }
                                                    None => {}
                                                }

                                                cell.right_wall = None;
                                            }
                                            StrictCellFace::Floor => {
                                                match &cell.floor {
                                                    Some(wall) => {
                                                        old_cell_entity = wall.entity;
                                                    }
                                                    None => {}
                                                }

                                                cell.floor = None;
                                            }
                                            StrictCellFace::Center => {
                                                match &cell.center {
                                                    Some(wall) => {
                                                        old_cell_entity = wall.entity;
                                                    }
                                                    None => {}
                                                }

                                                cell.center = None;
                                            }
                                        }

                                        match old_cell_entity {
                                            Some(ent) => {
                                                despawn.send(DespawnEntity { entity: ent });
                                            }
                                            None => {}
                                        }
                                        cell_empty = cell.is_empty();
                                    }
                                    None => {}
                                }
                                if cell_empty {
                                    *cell_option = None;
                                }
                            }
                            None => {}
                        }

                        clear_chunk = grid_chunk.is_empty();
                    }
                    None => {}
                }

                if clear_chunk {
                    *grid_chunk_option = None;
                }
            }
            None => {}
        }
    }
    for chunk_id in changed_chunks {
        let chunk_colliders = gridmap.get_chunk_collider_data(chunk_id);

        match gridmap.colliders.get(&chunk_id) {
            Some(chunk_collider_entity) => {
                match gridmap_collider_query.get_mut(*chunk_collider_entity) {
                    Ok((gridmap_collider_component, mut collider)) => {
                        if gridmap_collider_component.chunk_id == chunk_id {
                            *collider = Collider::compound(chunk_colliders);
                        }
                    }
                    Err(_) => {
                        warn!("No collider chunk found.");
                    }
                }
            }
            None => {
                warn!("Trying to remove a non-existing chunk.");
            }
        }
    }
}

pub(crate) fn add_tile_net(
    mut events: EventReader<AddTile>,
    connected_players: Query<&ConnectedPlayer, Without<SoftPlayer>>,
    mut net: EventWriter<OutgoingReliableServerMessage<GridmapServerMessage>>,
    mut gridmap: ResMut<Gridmap>,
) {
    for event in events.read() {
        let mut received = vec![];

        let target = TargetCell {
            id: event.id,
            face: event.face.clone(),
        };

        if !event.default_map_spawn {
            for connected_player in connected_players.iter() {
                if !connected_player.connected {
                    continue;
                }
                net.send(OutgoingReliableServerMessage {
                    handle: connected_player.handle,
                    message: GridmapServerMessage::AddCell(NewCell {
                        cell: LayerTargetCell {
                            target: target.clone(),
                            is_detail: event.is_detail,
                        },
                        orientation: event.orientation,
                        tile_type: event.tile_type,
                    }),
                });
                received.push(connected_player.handle);
            }
            gridmap.updates.insert(
                LayerTargetCell {
                    target: target.clone(),
                    is_detail: event.is_detail,
                },
                AddedUpdate {
                    cell: GridmapUpdate::Added(NewCell {
                        cell: LayerTargetCell {
                            target,
                            is_detail: event.is_detail,
                        },
                        orientation: event.orientation,
                        tile_type: event.tile_type,
                    }),
                    players_received: received,
                },
            );
        }
    }

    for connected_player in connected_players.iter() {
        if !connected_player.connected {
            return;
        }

        for (_target_cell, update) in gridmap.updates.iter_mut() {
            match &update.cell {
                GridmapUpdate::Added(add) => {
                    if !update.players_received.contains(&connected_player.handle) {
                        update.players_received.push(connected_player.handle);
                        net.send(OutgoingReliableServerMessage {
                            handle: connected_player.handle,
                            message: GridmapServerMessage::AddCell(NewCell {
                                cell: add.cell.clone(),
                                orientation: add.orientation,
                                tile_type: add.tile_type,
                            }),
                        });
                    }
                }
                _ => (),
            }
        }
    }
}

pub(crate) fn remove_tile_net(
    mut events: EventReader<RemoveTile>,
    connected_players: Query<&ConnectedPlayer, Without<SoftPlayer>>,
    mut net: EventWriter<OutgoingReliableServerMessage<GridmapServerMessage>>,
    mut gridmap: ResMut<Gridmap>,
) {
    for event in events.read() {
        let mut received = vec![];

        for connected_player in connected_players.iter() {
            if !connected_player.connected {
                continue;
            }
            net.send(OutgoingReliableServerMessage {
                handle: connected_player.handle,
                message: GridmapServerMessage::RemoveCell(event.cell.clone()),
            });
            received.push(connected_player.handle);
        }

        gridmap.updates.insert(
            event.cell.clone(),
            AddedUpdate {
                cell: GridmapUpdate::Removed,
                players_received: received,
            },
        );
    }

    for connected_player in connected_players.iter() {
        if !connected_player.connected {
            return;
        }

        for (target_cell, update) in gridmap.updates.iter_mut() {
            match update.cell {
                GridmapUpdate::Removed => {
                    if !update.players_received.contains(&connected_player.handle) {
                        update.players_received.push(connected_player.handle);
                        net.send(OutgoingReliableServerMessage {
                            handle: connected_player.handle,
                            message: GridmapServerMessage::RemoveCell(target_cell.clone()),
                        });
                    }
                }
                _ => (),
            }
        }
    }
}

pub(crate) fn add_cell_client(
    mut net: EventReader<IncomingReliableServerMessage<GridmapServerMessage>>,
    mut event: EventWriter<AddTile>,
    mut commands: Commands,
) {
    for message in net.read() {
        match &message.message {
            GridmapServerMessage::AddCell(new) => {
                event.send(AddTile {
                    id: new.cell.target.id,
                    tile_type: new.tile_type,
                    orientation: new.orientation,
                    face: new.cell.target.face.clone(),
                    group_instance_id_option: None,
                    entity: commands.spawn(()).id(),
                    is_detail: new.cell.is_detail,
                    default_map_spawn: false,
                });
            }
            _ => (),
        }
    }
}

pub(crate) fn remove_cell_client(
    mut net: EventReader<IncomingReliableServerMessage<GridmapServerMessage>>,
    mut event: EventWriter<RemoveTile>,
) {
    for message in net.read() {
        match &message.message {
            GridmapServerMessage::RemoveCell(new) => {
                event.send(RemoveTile { cell: new.clone() });
            }
            _ => {}
        }
    }
}

pub(crate) fn add_tile_collision(
    mut events: EventReader<AddTile>,
    mut commands: Commands,
    mut gridmap_data: ResMut<Gridmap>,
    mut gridmap_collider_query: Query<(&GridmapCollider, &mut Collider)>,
) {
    let mut changed_chunks = vec![];
    for event in events.read() {
        let chunk_id = gridmap_data.get_indexes(event.id).chunk;
        if !changed_chunks.contains(&chunk_id) {
            changed_chunks.push(chunk_id);
        }
    }
    for chunk_id in changed_chunks {
        let mut friction_component = Friction::new(STANDARD_BODY_FRICTION);
        friction_component.combine_rule = CoefficientCombine::Min;
        let masks = get_bit_masks(ColliderGroup::Standard);

        let chunk_colliders = gridmap_data.get_chunk_collider_data(chunk_id);

        match gridmap_data.colliders.get(&chunk_id) {
            Some(chunk_collider_entity) => {
                match gridmap_collider_query.get_mut(*chunk_collider_entity) {
                    Ok((gridmap_collider_component, mut collider)) => {
                        if gridmap_collider_component.chunk_id == chunk_id {
                            *collider = Collider::compound(chunk_colliders);
                        }
                    }
                    Err(_) => {
                        warn!("No collider chunk found.");
                    }
                }
            }
            None => {
                let new_chunk_id = commands
                    .spawn((
                        Collider::compound(chunk_colliders),
                        GridmapCollider { chunk_id: chunk_id },
                        friction_component,
                        RigidBody::Static,
                        CollisionLayers::from_bits(masks.0, masks.1),
                        TransformBundle::default(),
                    ))
                    .id();
                gridmap_data.colliders.insert(chunk_id, new_chunk_id);
            }
        }

        /*if is_server() || matches!(*app_mode, AppMode::Correction) {
            entity_builder.insert(TransformBundle::from(world_position));
        }*/
    }
}

pub(crate) fn add_tile(
    mut events: EventReader<AddTile>,
    mut gridmap_main: ResMut<Gridmap>,
    mut commands: Commands,
) {
    for add_tile_event in events.read() {
        commands.entity(add_tile_event.entity).insert((
            Tile,
            Cell {
                id: add_tile_event.id,
            },
        ));
        let strict = gridmap_main.get_strict_cell(TargetCell {
            id: add_tile_event.id,
            face: add_tile_event.face.clone(),
        });

        let indexes = gridmap_main.get_indexes(strict.id);
        let grid;
        if !add_tile_event.is_detail {
            grid = &mut gridmap_main.main_grid;
        } else {
            grid = &mut gridmap_main.details_grid;
        }
        match grid.get_mut(indexes.chunk) {
            Some(chunk_option) => {
                match chunk_option {
                    Some(_) => {}
                    None => {
                        *chunk_option = Some(GridmapChunk::default());
                    }
                }
                match chunk_option {
                    Some(chunk) => {
                        let mut y = chunk.cells.get_mut(indexes.cell);
                        let x = y.as_mut().unwrap();

                        match x {
                            Some(_) => {}
                            None => {
                                **x = Some(GridCell::default());
                            }
                        }

                        let grid_items = x.as_mut().unwrap();

                        let mut health_flags = HashMap::new();

                        health_flags.insert(0, HealthFlag::ArmourPlated);

                        let new = Some(CellItem {
                            tile_type: add_tile_event.tile_type,
                            entity: Some(add_tile_event.entity),
                            health: Health {
                                health_flags: health_flags.clone(),
                                health_container: HealthContainer::Structure(
                                    StructureHealth::default(),
                                ),
                                ..Default::default()
                            },
                            orientation: add_tile_event.orientation.clone(),
                            group_id_option: add_tile_event.group_instance_id_option,
                        });

                        match strict.face {
                            StrictCellFace::FrontWall => {
                                grid_items.front_wall = new;
                            }
                            StrictCellFace::RightWall => {
                                grid_items.right_wall = new;
                            }
                            StrictCellFace::Floor => {
                                grid_items.floor = new;
                            }
                            StrictCellFace::Center => {
                                grid_items.center = new;
                            }
                        }
                    }
                    None => {
                        warn!("No chunk option");
                        continue;
                    }
                }
            }
            None => {
                warn!("set_cell couldn't find chunk.");
            }
        }
    }
}

pub struct OrthogonalBases {
    pub bases: [Quat; 24],
}
impl Default for OrthogonalBases {
    fn default() -> Self {
        Self {
            bases: [
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    1., 0., 0., 0., 1., 0., 0., 0., 1.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., -1., 0., 1., 0., 0., 0., 0., 1.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    -1., 0., 0., 0., -1., 0., 0., 0., 1.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 1., 0., -1., 0., 0., 0., 0., 1.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    1., 0., 0., 0., 0., -1., 0., 1., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 0., 1., 1., 0., 0., 0., 1., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    -1., 0., 0., 0., 0., 1., 0., 1., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 0., -1., -1., 0., 0., 0., 1., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    1., 0., 0., 0., -1., 0., 0., 0., -1.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 1., 0., 1., 0., 0., 0., 0., -1.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    -1., 0., 0., 0., 1., 0., 0., 0., -1.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., -1., 0., -1., 0., 0., 0., 0., -1.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    1., 0., 0., 0., 0., 1., 0., -1., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 0., -1., 1., 0., 0., 0., -1., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    -1., 0., 0., 0., 0., -1., 0., -1., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 0., 1., -1., 0., 0., 0., -1., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 0., 1., 0., 1., 0., -1., 0., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., -1., 0., 0., 0., 1., -1., 0., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 0., -1., 0., -1., 0., -1., 0., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 1., 0., 0., 0., -1., -1., 0., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 0., 1., 0., -1., 0., 1., 0., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 1., 0., 0., 0., 1., 1., 0., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., 0., -1., 0., 1., 0., 1., 0., 0.,
                ])),
                Quat::from_mat3(&Mat3::from_cols_array(&[
                    0., -1., 0., 0., 0., -1., 1., 0., 0.,
                ])),
            ],
        }
    }
}

pub trait Orthogonal {
    fn get_orthogonal_index(&self) -> u8;
}

impl Orthogonal for Quat {
    fn get_orthogonal_index(&self) -> u8 {
        let bases = OrthogonalBases::default().bases;
        let mut math3_cols = Mat3::from_quat(*self).to_cols_array_2d();
        for i in 0..3 {
            for j in 0..3 {
                let mut v = math3_cols[i][j];
                if v > 0.5 {
                    v = 1.;
                } else if v < -0.5 {
                    v = -1.;
                } else {
                    v = 0.;
                }

                math3_cols[i][j] = v;
            }
        }

        for i in 0..24 {
            if bases[i] == *self {
                return i as u8;
            }
        }

        return 0;
    }
}

pub(crate) fn spawn_group(
    mut events: EventReader<AddGroup>,
    mut gridmap_main: ResMut<Gridmap>,
    mut set_tile: EventWriter<AddTile>,
    mut commands: Commands,
) {
    for add_group_event in events.read() {
        match gridmap_main.groups.get(&add_group_event.group_id) {
            Some(tiles) => {
                let mut i = 0;

                for (local_id, tile_type) in tiles.iter() {
                    let int = &Vec3Int { x: 0, y: 0, z: 0 };

                    let mut new_id = local_id.clone();

                    if local_id != int {
                        let mut point = Transform::from_translation(Vec3::new(
                            local_id.x as f32,
                            local_id.y as f32,
                            local_id.z as f32,
                        ));

                        point.rotate(
                            OrthogonalBases::default().bases[add_group_event.orientation as usize],
                        );

                        new_id = Vec3Int {
                            x: point.translation.x as i16,
                            y: point.translation.y as i16,
                            z: point.translation.z as i16,
                        }
                    }
                    let is_detail;

                    match gridmap_main.tile_properties.get(&tile_type.tile_type) {
                        Some(properties) => {
                            is_detail = properties.is_detail;
                        }
                        None => {
                            warn!("no tile properties found");
                            continue;
                        }
                    }

                    set_tile.send(AddTile {
                        id: add_group_event.id + new_id,
                        tile_type: tile_type.tile_type,
                        orientation: add_group_event.orientation,
                        face: add_group_event.face.clone(),
                        group_instance_id_option: Some(
                            gridmap_main.group_instance_incremental + i + 1,
                        ),
                        entity: commands.spawn(()).id(),
                        default_map_spawn: add_group_event.default_map_spawn,
                        is_detail: is_detail,
                    });
                    i += 1;
                }
                gridmap_main.group_instance_incremental += i;
            }
            None => {
                warn!("Couldnt find to be spawned group.");
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
pub struct TargetCellWithOrientationWType {
    pub id: Vec3Int,
    pub face: CellFace,
    pub orientation: u8,
    pub tile_type: CellTypeId,
}
