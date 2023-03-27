use std::{fs, path::Path};

use bevy::prelude::{info, warn, Commands, EventWriter, Res, ResMut, Resource};
use bevy_rapier3d::plugin::{RapierConfiguration, TimestepMode};
use resources::math::Vec3Int;
use resources::{core::TickRate, grid::CellFace};

use crate::grid::{
    AddGroup, AddTile, CellTypeId, CellTypeName, Gridmap, GroupTypeName, TileProperties,
};

/// Physics friction on placeable item surfaces.

//pub const PLACEABLE_SURFACE_FRICTION: f32 = 0.2;
/// Physics coefficient combiner of placeable item surfaces.

//pub const PLACEABLE_FRICTION: CoefficientCombineRule = CoefficientCombineRule::Min;

/// Initiate map resource meta-data.
#[derive(Default, Resource)]
pub struct InitTileProperties {
    pub properties: Vec<TileProperties>,
}

pub(crate) fn init_tile_properties(
    mut gridmap_data: ResMut<Gridmap>,
    init: Res<InitTileProperties>,
) {
    gridmap_data.non_fov_blocking_cells_list.push(CellTypeId(0));

    for cell_properties in init.properties.iter() {
        gridmap_data
            .main_text_names
            .insert(cell_properties.id, cell_properties.name.clone());
        gridmap_data
            .main_text_examine_desc
            .insert(cell_properties.id, cell_properties.description.clone());

        if cell_properties.non_fov_blocker {
            gridmap_data
                .non_fov_blocking_cells_list
                .push(cell_properties.id);
        }

        if !cell_properties.combat_obstacle {
            gridmap_data
                .non_combat_obstacle_cells_list
                .push(cell_properties.id)
        }

        if cell_properties.placeable_item_surface {
            gridmap_data
                .placeable_items_cells_list
                .push(cell_properties.id);
        }

        if !cell_properties.laser_combat_obstacle {
            gridmap_data
                .non_laser_obstacle_cells_list
                .push(cell_properties.id);
        }

        gridmap_data
            .main_cell_properties
            .insert(cell_properties.id, cell_properties.clone());
    }

    info!("Loaded {} gridmap cell types.", init.properties.len());
}
use player::spawn_points::SpawnPointRon;

/// Initiate other gridmap meta-datas from ron.

pub(crate) fn startup_misc_resources(
    mut gridmap_data: ResMut<Gridmap>,
    mut spawn_points_res: ResMut<SpawnPoints>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    tick_rate: Res<TickRate>,
) {
    // Init Bevy Rapier physics.

    rapier_configuration.timestep_mode = TimestepMode::Variable {
        max_dt: 1. / tick_rate.physics_rate as f32,
        time_scale: 1.,
        substeps: 1,
    };

    let mainordered_cells_ron = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("mainordered.ron");
    let current_map_mainordered_cells_raw_ron: String = fs::read_to_string(mainordered_cells_ron)
        .expect("Error reading map mainordered.ron drive.");
    let current_map_mainordered_cells: Vec<String> =
        ron::from_str(&current_map_mainordered_cells_raw_ron)
            .expect("Error parsing map mainordered.ron String.");

    let mut current_map_mainordered_cells_typed: Vec<CellTypeName> = vec![];

    for (i, name) in current_map_mainordered_cells.iter().rev().enumerate() {
        gridmap_data
            .main_name_id_map
            .insert(CellTypeName(name.to_string()), CellTypeId(i as u16));
        gridmap_data
            .main_id_name_map
            .insert(CellTypeId(i as u16), CellTypeName(name.to_string()));
        current_map_mainordered_cells_typed.push(CellTypeName(name.to_string()));
    }

    gridmap_data.ordered_main_names = current_map_mainordered_cells_typed;

    let spawnpoints_ron = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("spawnpoints.ron");
    let current_map_spawn_points_raw_ron: String =
        fs::read_to_string(spawnpoints_ron).expect("Error reading map spawnpoints.ron from drive.");
    let current_map_spawn_points_raw: Vec<SpawnPointRon> =
        ron::from_str(&current_map_spawn_points_raw_ron)
            .expect("Error parsing map spawnpoints.ron String.");
    let mut current_map_spawn_points: Vec<SpawnPoint> = vec![];

    for raw_point in current_map_spawn_points_raw.iter() {
        current_map_spawn_points.push(SpawnPoint::new(&raw_point.new()));
    }
    info!("Loaded {} spawnpoints.", current_map_spawn_points.len());
    spawn_points_res.list = current_map_spawn_points;
    spawn_points_res.i = 0;
}

/// Build the gridmaps in their own resources from ron.

pub(crate) fn load_ron_gridmap(
    gridmap_data: Res<Gridmap>,
    mut set_cell: EventWriter<AddTile>,
    mut set_group: EventWriter<AddGroup>,
    mut commands: Commands,
) {
    // Load map json data into real static bodies.
    let main_ron = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("main.bin");
    let current_map_main_raw_ron = fs::read(main_ron)
        .expect("startup_build_map() Error reading map main.ron file from drive.");

    if current_map_main_raw_ron.len() == 0 {
        warn!("Empty main.ron map file.");
        return;
    }

    let current_map_main_data: Vec<CellDataExport> =
        bincode::deserialize(&current_map_main_raw_ron)
            .expect("startup_build_map() Error parsing map main.ron String.");

    for cell_data in current_map_main_data.iter() {
        match &cell_data.item {
            ItemExport::Cell(item) => {
                let cell_item_id;

                match gridmap_data.main_name_id_map.get(item) {
                    Some(x) => {
                        cell_item_id = *x;
                    }
                    None => {
                        warn!("Couldnt find item {:?}", item);
                        break;
                    }
                };
                set_cell.send(AddTile {
                    id: cell_data.id,
                    face: cell_data.face.clone(),
                    orientation: cell_data.orientation.clone(),
                    tile_type: cell_item_id,
                    group_id_option: None,
                    entity: commands.spawn(()).id(),
                    default_map_spawn: true,
                });
            }
            ItemExport::Group(item) => {
                let group_id;

                match gridmap_data.group_id_map.get(&item.name) {
                    Some(id) => {
                        group_id = id;
                    }
                    None => {
                        warn!("Couldnt find group id.");
                        continue;
                    }
                }
                set_group.send(AddGroup {
                    id: cell_data.id,
                    group_id: *group_id,
                    orientation: cell_data.orientation.clone(),
                    face: cell_data.face.clone(),
                    default_map_spawn: true,
                });
            }
        }
    }

    info!("Spawned {} map cells.", current_map_main_data.len());
}

use player::boarding::{SpawnPoint, SpawnPoints};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CellDataExport {
    pub id: Vec3Int,
    /// Cell item id.
    pub item: ItemExport,
    /// Cell rotation.
    pub orientation: u8,
    pub face: CellFace,
}

#[derive(Serialize, Deserialize)]
pub enum ItemExport {
    Cell(CellTypeName),
    Group(GroupItem),
}
#[derive(Serialize, Deserialize)]
pub struct GroupItem {
    pub name: GroupTypeName,
    pub group_id: u32,
}
