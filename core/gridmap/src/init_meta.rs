use std::{collections::HashMap, fs, path::Path};

use api::{
    chat::EXAMINATION_EMPTY,
    data::{ServerId, TickRate},
};
use bevy::prelude::{info, Commands, EventWriter, Res, ResMut, Transform};
use bevy_rapier3d::{
    plugin::{RapierConfiguration, TimestepMode},
    prelude::{CoefficientCombineRule, Collider},
};
use entity::entity_data::{load_raw_map_entities, RawEntity, RawSpawnEvent, Server};
use examinable::examine::RichName;
use pawn::pawn::{SpawnPoint, SpawnPointRaw, SpawnPoints};

use crate::{
    build::{build_details1_gridmap, build_gridmap_floor_and_roof, build_main_gridmap},
    events::CellDataWID,
    fov::DoryenMap,
    grid::{
        AdjacentTileDirection, GridDirectionRotations, GridmapData, GridmapDetails1, GridmapMain,
        MainCellProperties,
    },
    plugin::Details1CellProperties,
};

/// Physics friction on placeable item surfaces.
pub const PLACEABLE_SURFACE_FRICTION: f32 = 0.2;
/// Physics coefficient combiner of placeable item surfaces.
pub const PLACEABLE_FRICTION: CoefficientCombineRule = CoefficientCombineRule::Min;

/// Initiate map resource meta-data.
pub(crate) fn startup_map_cells(mut gridmap_data: ResMut<GridmapData>) {
    gridmap_data.blackcell_blocking_id = *gridmap_data
        .main_name_id_map
        .get("blackCellBlocking")
        .unwrap();
    gridmap_data.blackcell_id = *gridmap_data.main_name_id_map.get("blackCell").unwrap();

    let mut main_cells_data = vec![];

    let mut default_isometry = Transform::identity();

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
        friction: PLACEABLE_SURFACE_FRICTION,
        combine_rule: PLACEABLE_FRICTION,
        ..Default::default()
    });
    let mut default_isometry = Transform::identity();

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
        friction: PLACEABLE_SURFACE_FRICTION,
        combine_rule: PLACEABLE_FRICTION,
        ..Default::default()
    });
    let mut default_isometry = Transform::identity();

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
        friction: PLACEABLE_SURFACE_FRICTION,
        combine_rule: PLACEABLE_FRICTION,
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

    let mut default_isometry = Transform::identity();
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
        friction: PLACEABLE_SURFACE_FRICTION,
        combine_rule: PLACEABLE_FRICTION,
        ..Default::default()
    });

    let mut default_isometry = Transform::identity();
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
        friction: PLACEABLE_SURFACE_FRICTION,
        combine_rule: PLACEABLE_FRICTION,
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

/// Initiate other gridmap meta-datas from json.
pub(crate) fn startup_misc_resources(
    mut server_id: ResMut<ServerId>,
    mut gridmap_data: ResMut<GridmapData>,
    mut spawn_points_res: ResMut<SpawnPoints>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    tick_rate: Res<TickRate>,
    mut commands: Commands,
) {
    // Init Bevy Rapier physics.
    rapier_configuration.timestep_mode = TimestepMode::Variable {
        max_dt: 1. / tick_rate.physics_rate as f32,
        time_scale: 1.,
        substeps: 1,
    };

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

    // So we have one reserved Id that isnt an entity for sure
    let server_component = Server;

    server_id.id = commands.spawn().insert(server_component).id();

    info!("Loaded misc map data.");
}

/// Build the gridmaps in their own resources from json.
pub(crate) fn startup_build_map(
    mut gridmap_main: ResMut<GridmapMain>,
    mut gridmap_details1: ResMut<GridmapDetails1>,
    mut gridmap_data: ResMut<GridmapData>,
    mut fov_map: ResMut<DoryenMap>,
    mut commands: Commands,
    mut raw_spawner: EventWriter<RawSpawnEvent>,
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

    build_gridmap_floor_and_roof(&mut commands);

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

    load_raw_map_entities(&current_map_entities_data, &mut raw_spawner);

    info!("Spawned {} entities.", current_map_entities_data.len());
}
