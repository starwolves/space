use std::{fs, path::Path};

use bevy::prelude::{info, warn, Commands, EventWriter, Res, ResMut, Resource};
use bevy_rapier3d::plugin::{RapierConfiguration, TimestepMode};
use resources::math::Vec3Int;
use resources::{core::TickRate, grid::CellFace};

use crate::grid::{AddTile, CellTypeId, CellTypeName, Gridmap, GroupTypeName, TileProperties};

/// Physics friction on placeable item surfaces.

//pub const PLACEABLE_SURFACE_FRICTION: f32 = 0.2;
/// Physics coefficient combiner of placeable item surfaces.

//pub const PLACEABLE_FRICTION: CoefficientCombineRule = CoefficientCombineRule::Min;

/// Initiate map resource meta-data.

#[derive(Default, Resource)]
pub struct InitTileProperties {
    pub properties: Vec<TileProperties>,
}

pub(crate) fn init_tile_properties(mut gridmap: ResMut<Gridmap>, init: Res<InitTileProperties>) {
    let mut current_map_mainordered_cells_typed = vec![];
    for properties in init.properties.iter() {
        let gri_id = CellTypeId(gridmap.tile_type_incremental);
        gridmap.tile_properties.insert(gri_id, properties.clone());

        gridmap
            .main_name_id_map
            .insert(properties.name_id.clone(), gri_id);
        gridmap
            .main_id_name_map
            .insert(gri_id, properties.name_id.clone());
        current_map_mainordered_cells_typed.push(properties.name_id.clone());

        gridmap.tile_type_incremental += 1;
    }
    gridmap.ordered_main_names = current_map_mainordered_cells_typed;
    info!("Loaded {} gridmap cell types.", init.properties.len());
}
use player::spawn_points::SpawnPointRon;

/// Initiate other gridmap meta-datas from ron.

pub(crate) fn startup_misc_resources(
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
    gridmap: Res<Gridmap>,
    mut set_cell: EventWriter<AddTile>,
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

                match gridmap.main_name_id_map.get(item) {
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
                    group_instance_id_option: None,
                    entity: commands.spawn(()).id(),
                    default_map_spawn: true,
                });
            }
            ItemExport::Group(item) => {
                let id;
                match gridmap.main_name_id_map.get(&item.cell) {
                    Some(n) => {
                        id = n;
                    }
                    None => {
                        warn!("couildnt find name");
                        continue;
                    }
                }

                set_cell.send(AddTile {
                    id: cell_data.id,
                    face: cell_data.face.clone(),
                    orientation: cell_data.orientation.clone(),
                    tile_type: *id,
                    group_instance_id_option: Some(item.group_id),
                    entity: commands.spawn(()).id(),
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
    pub cell: CellTypeName,
}
