use bevy::prelude::{ResMut};
use bevy_rapier3d::prelude::{ColliderPosition, ColliderShape};

use crate::space_core::{resources::gridmap_data::GridmapData};



#[derive(Clone)]
pub struct MainCellProperties {
    pub id : i64,
    pub name : String,
    pub description : String,
    pub non_fov_blocker : bool,
    pub combat_obstacle : bool,
    pub placeable_item_surface : bool,
    pub laser_combat_obstacle : bool,
    pub collider_shape : ColliderShape,
    pub collider_position : ColliderPosition,
}

impl Default for MainCellProperties {
    fn default() -> Self {
        Self {
            id : 0,
            name : "".to_string(),
            description : "".to_string(),
            non_fov_blocker : false,
            combat_obstacle : true,
            placeable_item_surface : false,
            laser_combat_obstacle: true,
            collider_shape : ColliderShape::cuboid(1., 1., 1.),
            collider_position: ColliderPosition::identity(),
        }
    }
}


#[allow(dead_code)]
pub struct Details1CellProperties {
    pub id : i64,
    pub name : String,
    pub description : String,
}

impl Default for Details1CellProperties {
    fn default() -> Self {
        Self {
            id : 0,
            name : "".to_string(),
            description : "".to_string(),
        }
    }
}

pub fn startup_init_gridmap_cells(
    mut gridmap_data : ResMut<GridmapData>,
) {

    
    gridmap_data.blackcell_blocking_id = *gridmap_data.main_name_id_map.get("blackCellBlocking").unwrap();
    gridmap_data.blackcell_id = *gridmap_data.main_name_id_map.get("blackCell").unwrap();

    let mut main_cells_data = vec![];

    let mut default_isometry = ColliderPosition::identity();

    default_isometry.translation.y = -0.5;

    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityDecoratedTable").unwrap(),
            name: "a decorated security table".to_string(),
            description: "A decorated security table.".to_string(),
            non_fov_blocker:true,
            combat_obstacle:false,
            placeable_item_surface:true,
            collider_shape: ColliderShape::cuboid(1., 0.5, 1.),
            collider_position: default_isometry,
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("blackCellBlocking").unwrap(),
            name: "INVISIBLECELL2".to_string(),
            description: "You cannot see what is there.".to_string(),
            non_fov_blocker: true,
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("genericWall1Corner").unwrap(),
            name: "an aluminum wall".to_string(),
            description: "An aluminum wall.".to_string(),
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityWallEnd").unwrap(),
            name: "an aluminum security wall".to_string(),
            description: "An aluminum wall. This one is painted with security department colors.".to_string(),
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityWallCorner").unwrap(),
            name: "an aluminum security wall".to_string(),
            description: "An aluminum wall. This one is painted with security department colors.".to_string(),
            ..Default::default()
        }
    );

    let mut default_isometry = ColliderPosition::identity();
    default_isometry.translation.y = -0.5;

    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityCounter1").unwrap(),
            name: "a security counter".to_string(),
            description: "A counter. This one is painted with security department colors.".to_string(),
            non_fov_blocker:true,
            combat_obstacle:false,
            placeable_item_surface:true,
            collider_shape: ColliderShape::cuboid(1., 0.5, 0.5),
            collider_position: default_isometry,
            ..Default::default()
        }
    );

    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityFloorColored").unwrap(),
            name: "an aluminum security floor".to_string(),
            description: "An aluminum floor. This one is painted with security department colors.".to_string(),
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityFloorStripedCorner2").unwrap(),
            name: "an aluminum security floor".to_string(),
            description: "An aluminum floor. This one is painted with security department colors.".to_string(),
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityFloorStripedCorner").unwrap(),
            name: "an aluminum security floor".to_string(),
            description: "An aluminum floor. This one is painted with security department colors.".to_string(),
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("genericWall1").unwrap(),
            name: "an aluminum wall".to_string(),
            description: "An aluminum wall.".to_string(),
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("genericFloor1").unwrap(),
            name: "an aluminum floor".to_string(),
            description: "An aluminum floor.".to_string(),
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("blackCell").unwrap(),
            name: "INVISIBLECELL".to_string(),
            description: "You cannot see what is there.".to_string(),
            non_fov_blocker: true,
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityWall").unwrap(),
            name: "an aluminum security wall".to_string(),
            description: "An aluminum wall. This one is painted with security department colors.".to_string(),
            ..Default::default()
        }
    );
    main_cells_data.push(
        MainCellProperties {
            id: *gridmap_data.main_name_id_map.get("securityFloorStriped").unwrap(),
            name: "an aluminum security floor".to_string(),
            description: "An aluminum floor. This one is painted with security department colors.".to_string(),
            ..Default::default()
        }
    );

    for cell_properties in main_cells_data.iter() {

        gridmap_data.main_text_names.insert(cell_properties.id, cell_properties.name.clone());
        gridmap_data.main_text_examine_desc.insert(cell_properties.id, cell_properties.description.clone());

        if cell_properties.non_fov_blocker {
            gridmap_data.non_fov_blocking_cells_list.push(cell_properties.id);
        }

        if !cell_properties.combat_obstacle {
            gridmap_data.non_combat_obstacle_cells_list.push(cell_properties.id)
        }

        if cell_properties.placeable_item_surface {
            gridmap_data.placeable_items_cells_list.push(cell_properties.id);
        }

        if !cell_properties.laser_combat_obstacle {
            gridmap_data.non_laser_obstacle_cells_list.push(cell_properties.id);
        }

        gridmap_data.main_cell_properties.insert(cell_properties.id, cell_properties.clone());

    }

    let mut details1_cells_data = vec![];

    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("starboyPoster1").unwrap(),
            name: "".to_string(),
            description: "A well-preserved ancient collectible pop music poster, it must be at least a thousand years old. \n\"Starboy\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("apc").unwrap(),
            name: "".to_string(),
            description: "An administrative personal computer (APC). Authorized personnel can use these computers to check on the status of the sub-systems this room utilises.".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("airExhaust").unwrap(),
            name: "".to_string(),
            description: "An air exhaust. Here to ventilate and circulate oxygen throughout the spaceship.".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("liquidDrain").unwrap(),
            name: "".to_string(),
            description: "A liquid drain. It transports liquids through dedicated piping to a different destination.".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster6").unwrap(),
            name: "".to_string(),
            description: "A Red Dragon poster. Here to remind you that the nation's surveillance systems have never been as effective and important as it is now. \n\"Always\nWatchful\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("EMPTY0").unwrap(),
            name: "".to_string(),
            description: "You cannot see what is there.".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster4").unwrap(),
            name: "".to_string(),
            description: "A Red Dragon poster for security personnel. \n\"I\nServe\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonPoster2").unwrap(),
            name: "".to_string(),
            description: "A poster. \n \"Colonise\nSpace\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonPoster1").unwrap(),
            name: "".to_string(),
            description: "A glorious Red Dragon poster. \n\"Hail our\nRed\nNation\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster3").unwrap(),
            name: "".to_string(),
            description: "A glorious Red Dragon poster for security personnel. This one has a famous picture printed on it from hundreds of years ago, the start of the great nation captured in a single picture. \n\"We\nRose\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster2").unwrap(),
            name: "".to_string(),
            description: "A glorious Red Dragon poster for security personnel. A nation to look up to with pride. \n\"Our\nFather\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("redDragonSecurityPoster1").unwrap(),
            name: "".to_string(),
            description: "A glorious Red Dragon poster for security personnel to remind you of the collective's might. \n\"Protect\nControl\nPrevent\nSecure\"".to_string()
        }
    );
    details1_cells_data.push(
        Details1CellProperties{
            id: *gridmap_data.details1_name_id_map.get("floorLight1").unwrap(),
            name: "".to_string(),
            description: "A fluorescent floor light.".to_string()
        }
    );


    for cell_properties in details1_cells_data.iter() {

        gridmap_data.details1_text_examine_desc.insert(cell_properties.id, cell_properties.description.clone());

    }


}
