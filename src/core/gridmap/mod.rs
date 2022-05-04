pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

use std::{collections::HashMap, fs, path::Path};

use bevy_app::{App, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::{
    schedule::{ParallelSystemDescriptorCoercion, SystemSet},
    system::{Commands, Res, ResMut},
};
use bevy_log::info;
use bevy_rapier3d::{
    prelude::{Collider, RapierConfiguration},
    rapier::prelude::ColliderPosition,
};
use bevy_transform::components::Transform;

use crate::{
    core::{
        entity::{
            components::Server,
            functions::{load_raw_map_entities::load_raw_map_entities, raw_entity::RawEntity},
            resources::EntityDataResource,
        },
        gridmap::{
            functions::{
                build_gridmap_floor::build_gridmap_floor,
                build_gridmap_from_data::{build_details1_gridmap, build_main_gridmap},
                examine_cell::EXAMINATION_EMPTY,
            },
            resources::{
                CellDataWID, DoryenMap, GridmapData, GridmapDetails1, GridmapMain, SpawnPoint,
                SpawnPointRaw,
            },
        },
        world_environment::resources::WorldEnvironmentRaw,
    },
    entities::sfx::ambience::ambience_sfx::AmbienceSfxBundle,
};

use self::{
    events::{NetGridmapUpdates, NetProjectileFOV, ProjectileFOV, RemoveCell},
    resources::SpawnPoints,
    systems::{
        gridmap_updates::gridmap_updates, projectile_fov::projectile_fov, remove_cell::remove_cell,
        senser_update_fov::senser_update_fov, sensing_ability::gridmap_sensing_ability,
    },
};

use super::{
    atmospherics::systems::{
        net_system::net_system, rigidbody_forces_atmospherics::AdjacentTileDirection,
    },
    configuration::resources::{ServerId, TickRate},
    entity::systems::broadcast_position_updates::INTERPOLATION_LABEL1,
    examinable::components::RichName,
    plugin::{PostUpdateLabels, StartupLabels, UpdateLabels},
    world_environment::resources::WorldEnvironment,
};

pub fn startup_build_map(
    mut gridmap_main: ResMut<GridmapMain>,
    mut gridmap_details1: ResMut<GridmapDetails1>,
    mut gridmap_data: ResMut<GridmapData>,
    entity_data: Res<EntityDataResource>,
    mut fov_map: ResMut<DoryenMap>,
    mut commands: Commands,
) {
    // Load map json data into real static bodies.
    let main_json = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("main.json");
    let current_map_main_raw_json: String = fs::read_to_string(main_json)
        .expect("main.rs launch_server() Error reading map main.json file from drive.");
    let current_map_main_data: Vec<CellDataWID> = serde_json::from_str(&current_map_main_raw_json)
        .expect("main.rs launch_server() Error parsing map main.json String.");

    build_gridmap_floor(&mut commands);

    build_main_gridmap(
        &current_map_main_data,
        &mut commands,
        &mut gridmap_main,
        &mut fov_map,
        &mut gridmap_data,
    );

    let details1_json = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("details1.json");
    let current_map_details1_raw_json: String = fs::read_to_string(details1_json)
        .expect("main.rs launch_server() Error reading map details1_json file from drive.");
    let current_map_details1_data: Vec<CellDataWID> =
        serde_json::from_str(&current_map_details1_raw_json)
            .expect("main.rs launch_server() Error parsing map details1_json String.");

    build_details1_gridmap(
        &current_map_details1_data,
        &mut gridmap_details1,
        &mut gridmap_data,
    );

    info!(
        "Spawned {} map cells.",
        current_map_main_data.len() + current_map_details1_data.len()
    );

    let entities_json = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("entities.json");
    let current_map_entities_raw_json: String = fs::read_to_string(entities_json)
        .expect("main.rs launch_server() Error reading map entities.json file from drive.");
    let current_map_entities_data: Vec<RawEntity> =
        serde_json::from_str(&current_map_entities_raw_json)
            .expect("main.rs launch_server() Error parsing map entities.json String.");

    load_raw_map_entities(&current_map_entities_data, &mut commands, &entity_data);

    info!("Spawned {} entities.", current_map_entities_data.len());
}

#[derive(Clone)]
pub struct MainCellProperties {
    pub id: i64,
    pub name: RichName,
    pub description: String,
    pub non_fov_blocker: bool,
    pub combat_obstacle: bool,
    pub placeable_item_surface: bool,
    pub laser_combat_obstacle: bool,
    pub collider: Collider,
    pub collider_position: ColliderPosition,
    pub constructable: bool,
    pub floor_cell: bool,
    pub atmospherics_blocker: bool,
    pub atmospherics_pushes_up: bool,
    pub direction_rotations: GridDirectionRotations,
}

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

impl Default for MainCellProperties {
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
            collider_position: ColliderPosition::identity(),
            constructable: false,
            floor_cell: false,
            atmospherics_blocker: true,
            atmospherics_pushes_up: false,
            direction_rotations: GridDirectionRotations::default_wall_rotations(),
        }
    }
}

#[allow(dead_code)]
pub struct Details1CellProperties {
    pub id: i64,
    pub name: RichName,
    pub description: String,
}

impl Default for Details1CellProperties {
    fn default() -> Self {
        Self {
            id: 0,
            name: Default::default(),
            description: "".to_string(),
        }
    }
}

pub fn startup_map_cells(mut gridmap_data: ResMut<GridmapData>) {
    gridmap_data.blackcell_blocking_id = *gridmap_data
        .main_name_id_map
        .get("blackCellBlocking")
        .unwrap();
    gridmap_data.blackcell_id = *gridmap_data.main_name_id_map.get("blackCell").unwrap();

    let mut main_cells_data = vec![];

    let mut default_isometry = ColliderPosition::identity();

    default_isometry.translation.y = -0.5;

    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("securityDecoratedTable")
            .unwrap(),
        name: RichName {
            name: "decorated security table".to_string(),
            n: false,
            the: false,
        },
        description: "A decorated security table.".to_string(),
        non_fov_blocker: true,
        combat_obstacle: false,
        placeable_item_surface: true,
        collider: Collider::cuboid(1., 0.5, 1.),
        collider_position: default_isometry,
        constructable: true,
        ..Default::default()
    });
    let mut default_isometry = ColliderPosition::identity();

    default_isometry.translation.y = -0.5;

    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("governmentDecoratedTable")
            .unwrap(),
        name: RichName {
            name: "decorated government table".to_string(),
            n: false,
            the: false,
        },
        description: "A decorated government table.".to_string(),
        non_fov_blocker: true,
        combat_obstacle: false,
        placeable_item_surface: true,
        collider: Collider::cuboid(1., 0.5, 1.),
        collider_position: default_isometry,
        constructable: true,
        ..Default::default()
    });
    let mut default_isometry = ColliderPosition::identity();

    default_isometry.translation.y = -0.5;

    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("bridgeDecoratedTable")
            .unwrap(),
        name: RichName {
            name: "decorated bridge table".to_string(),
            n: false,
            the: false,
        },
        description: "A decorated bridge table.".to_string(),
        non_fov_blocker: true,
        combat_obstacle: false,
        placeable_item_surface: true,
        collider: Collider::cuboid(1., 0.5, 1.),
        collider_position: default_isometry,
        constructable: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("blackCellBlocking")
            .unwrap(),
        name: RichName {
            name: "INVISIBLECELL2".to_string(),
            n: false,
            the: false,
        },
        description: EXAMINATION_EMPTY.to_string(),
        non_fov_blocker: true,
        constructable: false,
        ..Default::default()
    });

    let mut default_isometry = ColliderPosition::identity();
    default_isometry.translation.y = -0.5;

    let mut rotations = HashMap::new();
    rotations.insert(AdjacentTileDirection::Left, 0);
    rotations.insert(AdjacentTileDirection::Right, 0);
    rotations.insert(AdjacentTileDirection::Up, 16);
    rotations.insert(AdjacentTileDirection::Down, 16);

    let rotation_struct = GridDirectionRotations { data: rotations };

    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("securityCounter1")
            .unwrap(),
        name: RichName {
            name: "security counter".to_string(),
            n: false,
            the: false,
        },
        description: "This one is painted with security department colors.".to_string(),
        non_fov_blocker: true,
        combat_obstacle: false,
        placeable_item_surface: true,
        collider: Collider::cuboid(1., 0.5, 0.5),
        collider_position: default_isometry,
        constructable: true,
        atmospherics_blocker: false,
        atmospherics_pushes_up: true,
        direction_rotations: rotation_struct,
        ..Default::default()
    });

    let mut default_isometry = ColliderPosition::identity();
    default_isometry.translation.y = -0.5;

    let mut rotations = HashMap::new();
    rotations.insert(AdjacentTileDirection::Left, 0);
    rotations.insert(AdjacentTileDirection::Right, 0);
    rotations.insert(AdjacentTileDirection::Up, 16);
    rotations.insert(AdjacentTileDirection::Down, 16);

    let rotation_struct = GridDirectionRotations { data: rotations };

    main_cells_data.push(MainCellProperties {
        id: *gridmap_data.main_name_id_map.get("bridgeCounter").unwrap(),
        name: RichName {
            name: "bridge counter".to_string(),
            n: false,
            the: false,
        },
        description: "This one is painted with bridge department colors.".to_string(),
        non_fov_blocker: true,
        combat_obstacle: false,
        placeable_item_surface: true,
        collider: Collider::cuboid(1., 0.5, 0.5),
        collider_position: default_isometry,
        constructable: true,
        atmospherics_blocker: false,
        atmospherics_pushes_up: true,
        direction_rotations: rotation_struct,
        ..Default::default()
    });

    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("securityFloorColored")
            .unwrap(),
        name: RichName {
            name: "aluminum security floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with security department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("bridgeFloorColored")
            .unwrap(),
        name: RichName {
            name: "aluminum bridge floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with bridge department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("governmentFloorColored")
            .unwrap(),
        name: RichName {
            name: "aluminum government floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with government department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("securityFloorStripedCorner2")
            .unwrap(),
        name: RichName {
            name: "aluminum security floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with security department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("bridgeFloorStripedCorner2")
            .unwrap(),
        name: RichName {
            name: "aluminum bridge floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with bridge department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("governmentFloorStripedCorner2")
            .unwrap(),
        name: RichName {
            name: "aluminum government floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with government department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("securityFloorStripedCorner")
            .unwrap(),
        name: RichName {
            name: "aluminum security floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with security department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("bridgeFloorStripedCorner")
            .unwrap(),
        name: RichName {
            name: "bridge security floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with bridge department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("governmentFloorStripedCorner")
            .unwrap(),
        name: RichName {
            name: "government security floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with government department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data.main_name_id_map.get("genericWall1").unwrap(),
        name: RichName {
            name: "aluminum wall".to_string(),
            n: true,
            the: false,
        },
        description: "A generic wall tile.".to_string(),
        constructable: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("reinforcedGlassWall")
            .unwrap(),
        name: RichName {
            name: "reinforced glass wall".to_string(),
            n: true,
            the: false,
        },
        description: "A transparent reinforced glass wall.".to_string(),
        constructable: true,
        non_fov_blocker: true,
        laser_combat_obstacle: false,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data.main_name_id_map.get("bridgeWall").unwrap(),
        name: RichName {
            name: "bridge wall".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with bridge department colors.".to_string(),
        constructable: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data.main_name_id_map.get("governmentWall").unwrap(),
        name: RichName {
            name: "bridge wall".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with bridge department colors.".to_string(),
        constructable: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data.main_name_id_map.get("securityWall").unwrap(),
        name: RichName {
            name: "aluminum security wall".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with security department colors.".to_string(),
        constructable: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data.main_name_id_map.get("bridgeWall").unwrap(),
        name: RichName {
            name: "aluminum bridge wall".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with bridge department colors.".to_string(),
        constructable: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("securityFloorStriped")
            .unwrap(),
        name: RichName {
            name: "aluminum security floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with security department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("bridgeFloorStriped")
            .unwrap(),
        name: RichName {
            name: "aluminum bridge floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with bridge department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data
            .main_name_id_map
            .get("governmentFloorStriped")
            .unwrap(),
        name: RichName {
            name: "aluminum government floor".to_string(),
            n: true,
            the: false,
        },
        description: "This one is painted with government department colors.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data.main_name_id_map.get("genericFloor1").unwrap(),
        name: RichName {
            name: "aluminum floor".to_string(),
            n: true,
            the: false,
        },
        description: "A generic floor tile.".to_string(),
        constructable: true,
        floor_cell: true,
        ..Default::default()
    });
    main_cells_data.push(MainCellProperties {
        id: *gridmap_data.main_name_id_map.get("blackCell").unwrap(),
        name: RichName {
            name: "INVISIBLECELL".to_string(),
            n: true,
            the: false,
        },
        description: EXAMINATION_EMPTY.to_string(),
        non_fov_blocker: true,
        constructable: false,
        ..Default::default()
    });

    gridmap_data.non_fov_blocking_cells_list.push(-1);

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

    let mut details1_cells_data = vec![];

    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("starboyPoster1").unwrap(),
            name: RichName {
                name: "pop poster".to_string() ,
                n: false,
                the: false,
            },
            description: "A well-preserved ancient collectible pop music poster, it must be at least a thousand years old. \n\"Starboy\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("apc").unwrap(),
            name: RichName {
                name: "APC".to_string() ,
                n: true,
                the: false,
            },
            description: "An administrative personal computer (APC). Authorized personnel can use these computers to check on the status of the sub-systems this room utilises.".to_string()
        }
    );
    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data.details1_name_id_map.get("airExhaust").unwrap(),
        name: RichName {
            name: "air exhaust".to_string(),
            n: true,
            the: false,
        },
        description:
            "An air exhaust. Here to ventilate and circulate oxygen throughout the spaceship."
                .to_string(),
    });
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("liquidDrain").unwrap(),
            name: RichName {
                name: "liquid drain".to_string() ,
                n: true,
                the: false,
            },
            description: "A liquid drain. It transports liquids through dedicated piping to a different destination.".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster6").unwrap(),
            name: RichName {
                name: "security poster".to_string() ,
                n: false,
                the: false,
            },
            description: "A Red Dragon poster. Here to remind you that the nation's surveillance systems have never been as effective and important as it is now. \n\"Always\nWatchful\"".to_string()
        }
    );
    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data.details1_name_id_map.get("EMPTY0").unwrap(),
        name: RichName {
            name: "INVISIBLEDCELL1".to_string(),
            n: true,
            the: false,
        },
        description: EXAMINATION_EMPTY.to_string(),
    });
    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data
            .details1_name_id_map
            .get("redDragonSecurityPoster4")
            .unwrap(),
        name: RichName {
            name: "security poster".to_string(),
            n: false,
            the: false,
        },
        description: "A Red Dragon poster for security personnel. \n\"I\nServe\"".to_string(),
    });
    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data
            .details1_name_id_map
            .get("redDragonPoster2")
            .unwrap(),
        name: RichName {
            name: "poster".to_string(),
            n: false,
            the: false,
        },
        description: "A poster. \n \"Colonise\nSpace\"".to_string(),
    });
    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data
            .details1_name_id_map
            .get("redDragonPoster1")
            .unwrap(),
        name: RichName {
            name: "poster".to_string(),
            n: false,
            the: false,
        },
        description: "A glorious Red Dragon poster. \n\"Hail our\nRed\nNation\"".to_string(),
    });
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster3").unwrap(),
            name: RichName {
                name: "security poster".to_string() ,
                n: true,
                the: false,
            },
            description: "A glorious Red Dragon poster for security personnel. This one has a famous picture printed on it from hundreds of years ago, the start of the great nation captured in a single picture. \n\"We\nRose\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster2").unwrap(),
            name: RichName {
                name: "security poster".to_string() ,
                n: false,
                the: false,
            },
            description: "A glorious Red Dragon poster for security personnel. A nation to look up to with pride. \n\"Our\nFather\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster1").unwrap(),
            name: RichName {
                name: "security poster".to_string() ,
                n: false,
                the: false,
            },
            description: "A glorious Red Dragon poster for security personnel to remind you of the collective's might. \n\"Protect\nControl\nPrevent\nSecure\"".to_string()
        }
    );

    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data
            .details1_name_id_map
            .get("redDragonBridgePoster1")
            .unwrap(),
        name: RichName {
            name: "bridge poster".to_string(),
            n: false,
            the: false,
        },
        description: "A poster for bridge personnel to remind you to lead. \n\"Take Charge\""
            .to_string(),
    });
    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data
            .details1_name_id_map
            .get("redDragonBridgePoster2")
            .unwrap(),
        name: RichName {
            name: "bridge poster".to_string(),
            n: false,
            the: false,
        },
        description: "A poster for bridge personnel showing artwork of the moon from back home."
            .to_string(),
    });
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonBridgePoster3").unwrap(),
            name: RichName {
                name: "bridge poster".to_string() ,
                n: false,
                the: false,
            },
            description: "A poster for bridge personnel to showcase the utmost importance of sharing data. \n\"Broadcast\nStream\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonBridgePoster4").unwrap(),
            name: RichName {
                name: "bridge poster".to_string() ,
                n: false,
                the: false,
            },
            description: "A poster for bridge personnel showing a brand new space fighter, usually carried on-board of large flagships. \n\"Vigilant\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonBridgePoster5").unwrap(),
            name: RichName {
                name: "bridge poster".to_string() ,
                n: false,
                the: false,
            },
            description: "A poster for bridge personnel reminding the importance of the connectivity of technology. The logo disturbingly reminds you of chains. \n\"Connect\"".to_string()
        }
    );
    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data
            .details1_name_id_map
            .get("redDragonBridgePoster6")
            .unwrap(),
        name: RichName {
            name: "bridge poster".to_string(),
            n: false,
            the: false,
        },
        description: "A poster for bridge personnel. \n\"Remotely connected\"".to_string(),
    });
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonBridgePoster7").unwrap(),
            name: RichName {
                name: "bridge poster".to_string() ,
                n: false,
                the: false,
            },
            description: "A poster for bridge personnel promoting its staff to be watchful with the help of security cameras installed around the ship. \n\"Watchful\"".to_string()
        }
    );

    details1_cells_data.push(Details1CellProperties {
        id: *gridmap_data
            .details1_name_id_map
            .get("floorLight1")
            .unwrap(),
        name: RichName {
            name: "fluorescent floor light".to_string(),
            n: true,
            the: false,
        },
        description: "A fluorescent floor light.".to_string(),
    });

    for cell_properties in details1_cells_data.iter() {
        gridmap_data
            .details1_text_names
            .insert(cell_properties.id, cell_properties.name.clone());
        gridmap_data
            .details1_text_examine_desc
            .insert(cell_properties.id, cell_properties.description.clone());
    }

    info!(
        "Loaded {} different map cell types.",
        main_cells_data.len() + details1_cells_data.len()
    );
}

pub fn startup_misc_resources(
    mut server_id: ResMut<ServerId>,
    mut map_environment: ResMut<WorldEnvironment>,
    mut gridmap_data: ResMut<GridmapData>,
    mut spawn_points_res: ResMut<SpawnPoints>,
    mut _rapier_configuration: ResMut<RapierConfiguration>,
    _tick_rate: Res<TickRate>,
    mut commands: Commands,
) {
    // Init Bevy Rapier physics.
    //rapier_configuration.timestep_mode = TimestepMode::VariableTimestep;
    //rapier_integration_params.dt = 1. / tick_rate.rate as f32;

    let environment_json_location = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("environment.json");
    let current_map_environment_raw_json: String = fs::read_to_string(environment_json_location)
        .expect("main.rs main() Error reading map environment.json file from drive.");
    let current_map_raw_environment: WorldEnvironmentRaw =
        serde_json::from_str(&current_map_environment_raw_json)
            .expect("main.rs main() Error parsing map environment.json String.");
    let current_map_environment: WorldEnvironment =
        WorldEnvironment::new(current_map_raw_environment);

    current_map_environment.adjust(&mut map_environment);

    let mainordered_cells_json = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("mainordered.json");
    let current_map_mainordered_cells_raw_json: String = fs::read_to_string(mainordered_cells_json)
        .expect("main.rs main() Error reading map mainordered.json drive.");
    let current_map_mainordered_cells: Vec<String> =
        serde_json::from_str(&current_map_mainordered_cells_raw_json)
            .expect("main.rs main() Error parsing map mainordered.json String.");

    let details1ordered_cells_json = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("details1ordered.json");
    let current_map_details1ordered_cells_raw_json: String =
        fs::read_to_string(details1ordered_cells_json)
            .expect("main.rs main() Error reading map details1ordered.json drive.");
    let current_map_details1ordered_cells: Vec<String> =
        serde_json::from_str(&current_map_details1ordered_cells_raw_json)
            .expect("main.rs main() Error parsing map details1ordered.json String.");

    for (i, name) in current_map_mainordered_cells.iter().rev().enumerate() {
        gridmap_data
            .main_name_id_map
            .insert(name.to_string(), i as i64);
        gridmap_data
            .main_id_name_map
            .insert(i as i64, name.to_string());
    }

    for (i, name) in current_map_details1ordered_cells.iter().rev().enumerate() {
        gridmap_data
            .details1_name_id_map
            .insert(name.to_string(), i as i64);
        gridmap_data
            .details1_id_name_map
            .insert(i as i64, name.to_string());
    }

    gridmap_data.ordered_main_names = current_map_mainordered_cells;
    gridmap_data.ordered_details1_names = current_map_details1ordered_cells;

    let spawnpoints_json = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("spawnpoints.json");
    let current_map_spawn_points_raw_json: String = fs::read_to_string(spawnpoints_json)
        .expect("main.rs main() Error reading map spawnpoints.json from drive.");
    let current_map_spawn_points_raw: Vec<SpawnPointRaw> =
        serde_json::from_str(&current_map_spawn_points_raw_json)
            .expect("main.rs main() Error parsing map spawnpoints.json String.");
    let mut current_map_spawn_points: Vec<SpawnPoint> = vec![];

    for raw_point in current_map_spawn_points_raw.iter() {
        current_map_spawn_points.push(SpawnPoint::new(raw_point));
    }

    spawn_points_res.list = current_map_spawn_points;
    spawn_points_res.i = 0;

    // Spawn ambience SFX
    commands
        .spawn()
        .insert_bundle(AmbienceSfxBundle::new(Transform::identity()));

    // So we have one reserved Id that isnt an entity for sure
    let server_component = Server;

    server_id.id = commands.spawn().insert(server_component).id();

    info!("Loaded misc map data.");
}

pub struct GridmapPlugin;

impl Plugin for GridmapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridmapDetails1>()
            .init_resource::<GridmapData>()
            .init_resource::<DoryenMap>()
            .init_resource::<SpawnPoints>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(gridmap_updates),
            )
            .add_event::<NetGridmapUpdates>()
            .add_event::<ProjectileFOV>()
            .add_system(senser_update_fov)
            .add_system(projectile_fov)
            .add_system(remove_cell.label(UpdateLabels::DeconstructCell))
            .add_event::<NetProjectileFOV>()
            .add_event::<RemoveCell>()
            .add_startup_system(startup_misc_resources.label(StartupLabels::MiscResources))
            .add_startup_system(
                startup_map_cells
                    .label(StartupLabels::InitDefaultGridmapData)
                    .after(StartupLabels::MiscResources),
            )
            .init_resource::<GridmapDetails1>()
            .add_startup_system(
                startup_build_map
                    .label(StartupLabels::BuildGridmap)
                    .after(StartupLabels::InitDefaultGridmapData),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1. / 4.))
                    .with_system(gridmap_updates),
            )
            .init_resource::<GridmapMain>()
            .add_system_to_stage(
                PostUpdate,
                net_system.after(PostUpdateLabels::VisibleChecker),
            )
            .add_system(gridmap_sensing_ability);
    }
}
use bevy_app::CoreStage::PostUpdate;
