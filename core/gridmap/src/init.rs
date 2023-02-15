use std::{collections::HashMap, fs, path::Path};

use bevy::prelude::{info, warn, AssetServer, EventWriter, Res, ResMut, Transform};
use bevy_rapier3d::plugin::{RapierConfiguration, TimestepMode};
use entity::examine::RichName;
use math::grid::Vec3Int;
use resources::{core::TickRate, grid::CellFace, is_server::is_server};

use crate::grid::{AddGroup, AddTile, Gridmap, Orientation, TileProperties};

/// Physics friction on placeable item surfaces.

//pub const PLACEABLE_SURFACE_FRICTION: f32 = 0.2;
/// Physics coefficient combiner of placeable item surfaces.

//pub const PLACEABLE_FRICTION: CoefficientCombineRule = CoefficientCombineRule::Min;

/// Initiate map resource meta-data.

pub(crate) fn startup_map_tile_properties(
    mut gridmap_data: ResMut<Gridmap>,
    assets: Res<AssetServer>,
) {
    let mut main_cells_data = vec![];

    let mut default_isometry = Transform::IDENTITY;

    default_isometry.translation.y = -0.5;

    let mesh_option;
    if !is_server() {
        mesh_option = Some(assets.load("models/wall/wall.glb#Scene0"));
    } else {
        mesh_option = None;
    }
    main_cells_data.push(TileProperties {
        id: *gridmap_data.main_name_id_map.get("generic_wall_1").unwrap(),
        name: RichName {
            name: "aluminum wall".to_string(),
            n: true,
            the: false,
        },
        description: "A generic wall tile.".to_string(),
        constructable: true,
        mesh_option,
        ..Default::default()
    });
    let mut wall_group = HashMap::new();
    wall_group.insert(
        Vec3Int { x: 0, y: 0, z: 0 },
        *gridmap_data.main_name_id_map.get("generic_wall_1").unwrap(),
    );
    wall_group.insert(
        Vec3Int { x: 0, y: 1, z: 0 },
        *gridmap_data.main_name_id_map.get("generic_wall_1").unwrap(),
    );
    gridmap_data.groups.push(wall_group);
    gridmap_data
        .group_id_map
        .insert("generic_wall_group_1".to_string(), 0);
    gridmap_data
        .id_group_map
        .insert(0, "generic_wall_group_1".to_string());
    let mesh_option;
    if !is_server() {
        mesh_option = Some(assets.load("models/floor/floor.glb#Scene0"));
    } else {
        mesh_option = None;
    }
    main_cells_data.push(TileProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("generic_floor_1")
            .unwrap(),
        name: RichName {
            name: "aluminum floor".to_string(),
            n: true,
            the: false,
        },
        description: "A generic floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        mesh_option,
        ..Default::default()
    });

    gridmap_data.non_fov_blocking_cells_list.push(0);

    for cell_properties in main_cells_data.iter() {
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

    info!("Loaded {} gridmap cell types.", main_cells_data.len());
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

    for (i, name) in current_map_mainordered_cells.iter().rev().enumerate() {
        gridmap_data
            .main_name_id_map
            .insert(name.to_string(), i as u16);
        gridmap_data
            .main_id_name_map
            .insert(i as u16, name.to_string());
    }

    gridmap_data.ordered_main_names = current_map_mainordered_cells;

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
) {
    // Load map json data into real static bodies.
    let main_ron = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("main.ron");
    let current_map_main_raw_ron: String = fs::read_to_string(main_ron)
        .expect("startup_build_map() Error reading map main.ron file from drive.");

    if current_map_main_raw_ron.len() == 0 {
        warn!("Empty main.ron map file.");
        return;
    }

    let current_map_main_data: Vec<CellDataRon> = ron::from_str(&current_map_main_raw_ron)
        .expect("startup_build_map() Error parsing map main.ron String.");

    for cell_data in current_map_main_data.iter() {
        match &cell_data.item {
            RonItem::Cell(item) => {
                let cell_item_id;

                match gridmap_data.main_name_id_map.get(item) {
                    Some(x) => {
                        cell_item_id = *x;
                    }
                    None => {
                        warn!("Couldnt find item {}", item);
                        break;
                    }
                };

                set_cell.send(AddTile {
                    id: cell_data.id,
                    face: cell_data.face.clone(),
                    orientation_option: cell_data.orientation.clone(),
                    tile_type: cell_item_id,
                    group_instance_id_option: None,
                });
            }
            RonItem::Group(item) => {
                let group_item_id;

                match gridmap_data.group_id_map.get(item) {
                    Some(x) => {
                        group_item_id = *x;
                    }
                    None => {
                        warn!("Couldnt find item {}", item);
                        break;
                    }
                };

                set_group.send(AddGroup {
                    id: cell_data.id,
                    group_id: group_item_id,
                    orientation: cell_data.orientation.clone(),
                    face: cell_data.face.clone(),
                });
            }
        }
    }

    info!("Spawned {} map cells.", current_map_main_data.len());
}

use player::boarding::{SpawnPoint, SpawnPoints};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CellDataRon {
    pub id: Vec3Int,
    /// Cell item id.
    pub item: RonItem,
    /// Cell rotation.
    pub orientation: Option<Orientation>,
    pub face: CellFace,
}

#[derive(Serialize, Deserialize)]
pub enum RonItem {
    Cell(String),
    Group(String),
}
