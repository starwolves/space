use std::collections::HashMap;

use bevy::prelude::{Commands, Entity, ResMut, info};
use bevy_rapier3d::prelude::{
    CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderType,
    InteractionGroups, RigidBodyBundle, RigidBodyType,
};

use crate::space::core::{
    entity::functions::string_to_type_converters::string_vec3_to_vec3,
    gridmap::{
        components::Cell,
        resources::{
            to_doryen_coordinates, CellData, CellDataWID, DoryenMap, GridmapData, GridmapDetails1,
            GridmapMain, StructureHealth, Vec3Int,
        },
    },
    health::components::HealthFlag,
    physics::functions::{get_bit_masks, ColliderGroup},
};

use super::gridmap_functions::cell_id_to_world;

pub fn build_main_gridmap(
    current_map_main_data: &Vec<CellDataWID>,
    mut commands: &mut Commands,
    gridmap_main: &mut ResMut<GridmapMain>,
    fov_map: &mut ResMut<DoryenMap>,
    gridmap_data: &mut ResMut<GridmapData>,
) {
    let mut health_flags = HashMap::new();

    health_flags.insert(0, HealthFlag::ArmourPlated);

    for cell_data in current_map_main_data.iter() {
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let cell_id_int = Vec3Int {
            x: cell_id.x as i16,
            y: cell_id.y as i16,
            z: cell_id.z as i16,
        };

        let cell_item_id = *gridmap_data.main_name_id_map.get(&cell_data.item).unwrap();

        if &cell_data.item== "securityCounter1" {
            info!("{:?}", cell_data.orientation);
        }

        if cell_id_int.y == 0 {
            // Wall

            if !gridmap_data
                .non_fov_blocking_cells_list
                .contains(&cell_item_id)
            {
                let coords = to_doryen_coordinates(cell_id_int.x, cell_id_int.z);
                fov_map.map.set_transparent(coords.0, coords.1, false);
            }
        } else {
            // Floor cells dont have collision. Don't need to be an entity at this moment either.
            gridmap_main.data.insert(
                cell_id_int,
                CellData {
                    item: cell_item_id,
                    orientation: cell_data.orientation,
                    health: StructureHealth {
                        health_flags: health_flags.clone(),
                        ..Default::default()
                    },
                    entity: None,
                },
            );
            continue;
        }

        let entity_op = spawn_main_cell(
            &mut commands,
            cell_id_int,
            cell_item_id,
            cell_data.orientation,
            &gridmap_data,
        );

        gridmap_main.data.insert(
            cell_id_int,
            CellData {
                item: cell_item_id,
                orientation: cell_data.orientation,
                health: StructureHealth {
                    health_flags: health_flags.clone(),
                    ..Default::default()
                },
                entity: Some(entity_op),
            },
        );
    }
}

pub fn build_details1_gridmap(
    current_map_details1_data: &Vec<CellDataWID>,
    gridmap_details1: &mut ResMut<GridmapDetails1>,
    gridmap_data: &mut ResMut<GridmapData>,
) {
    for cell_data in current_map_details1_data.iter() {
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let cell_id_int = Vec3Int {
            x: cell_id.x as i16,
            y: cell_id.y as i16,
            z: cell_id.z as i16,
        };

        gridmap_details1.data.insert(
            cell_id_int,
            CellData {
                item: *gridmap_data
                    .details1_name_id_map
                    .get(&cell_data.item)
                    .unwrap(),
                orientation: cell_data.orientation,
                health: StructureHealth::default(),
                entity: None,
            },
        );
    }
}

// We also build cells in systems/construction_tool.rs
pub fn spawn_main_cell(
    commands: &mut Commands,
    cell_id: Vec3Int,
    cell_item_id: i64,
    _cell_rotation: i64,
    gridmap_data: &GridmapData,
) -> Entity {
    let world_position = cell_id_to_world(cell_id);

    let mut entity_builder = commands.spawn_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Static.into(),
        position: world_position.into(),
        ..Default::default()
    });

    let entity_id = entity_builder.id();

    let friction;
    let friction_combine_rule;

    if gridmap_data
        .placeable_items_cells_list
        .contains(&cell_item_id)
    {
        friction = 0.2;
        friction_combine_rule = CoefficientCombineRule::Min;
    } else {
        friction_combine_rule = CoefficientCombineRule::Min;
        friction = 0.;
    }

    let cell_properties = gridmap_data
        .main_cell_properties
        .get(&cell_item_id)
        .unwrap();

    let masks = get_bit_masks(ColliderGroup::Standard);

    entity_builder
        .insert_bundle(ColliderBundle {
            shape: cell_properties.collider_shape.clone().into(),
            position: cell_properties.collider_position.into(),
            collider_type: ColliderType::Solid.into(),
            material: ColliderMaterial {
                friction_combine_rule: friction_combine_rule,
                friction: friction,
                ..Default::default()
            }
            .into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(Cell { id: cell_id });

    entity_id
}
