use bevy::prelude::{Commands, ResMut};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyCcd, RigidBodyType};

use crate::space_core::{components::ship_cell::ShipCell, functions::{converters::string_to_type_converters::string_vec3_to_vec3, entity::collider_interaction_groups::{ColliderGroup, get_bit_masks}, gridmap::gridmap_functions::cell_id_to_world}, resources::{all_ordered_cells::AllOrderedCells, doryen_fov::{DoryenMap, to_doryen_coordinates}, gridmap_details1::GridmapDetails1, gridmap_main::{CellData, CellDataWID, GridmapMain}, network_messages::GridMapType, non_blocking_cells_list::NonBlockingCellsList, precalculated_fov_data::Vec3Int}};








pub fn build_main(
    current_map_main_data : &Vec<CellDataWID>, 
    commands : &mut Commands, 
    all_ordered_cells : &AllOrderedCells,
    gridmap_main : &mut ResMut<GridmapMain>,
    fov_map : &mut ResMut<DoryenMap>,
    non_fov_blocking_cells : &mut ResMut<NonBlockingCellsList>,
) {

    for cell_data in current_map_main_data.iter() {
        
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let cell_id_int = Vec3Int{
            x: cell_id.x as i16,
            y: cell_id.y as i16,
            z: cell_id.z as i16,
        };

        let world_position = cell_id_to_world(cell_id_int);

        gridmap_main.data.insert(cell_id_int,
        CellData {
            item: cell_data.item,
            orientation: cell_data.orientation,
        });


        if cell_id_int.y == 0 {
            // Wall

            if !non_fov_blocking_cells.list.contains(&cell_data.item) {
                let coords = to_doryen_coordinates(cell_id_int.x, cell_id_int.z);
                fov_map.map.set_transparent(coords.0, coords.1, false);
            }


        }
        
        

        if all_ordered_cells.main[((all_ordered_cells.main.len()-1) - cell_data.item as usize) as usize] == "securityCounter1" {

            let masks = get_bit_masks(ColliderGroup::StandardFOV);

            commands.spawn_bundle(RigidBodyBundle {
                body_type: RigidBodyType::Static,
                position: world_position.into(),
                ccd: RigidBodyCcd {
                    ccd_enabled: false,
                    ..Default::default()
                },
                ..Default::default()
            }).insert_bundle(
                ColliderBundle {
                    shape: ColliderShape::cuboid(1., 0.5, 0.5),
                    collider_type: ColliderType::Solid,
                    material: ColliderMaterial {
                        friction_combine_rule:  CoefficientCombineRule::Min,
                        ..Default::default()
                    },
                    flags: ColliderFlags {
                        collision_groups: InteractionGroups::new(masks.0,masks.1),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ).insert_bundle((
                ShipCell{
                    item: cell_data.item,
                    id: cell_id_int,
                    grid_type: GridMapType::Main,
                },
            ));

        } else {
            
            let masks = get_bit_masks(ColliderGroup::StandardFOV);

            commands.spawn_bundle(RigidBodyBundle {
                body_type: RigidBodyType::Static,
                position: world_position.into(),
                ccd: RigidBodyCcd {
                    ccd_enabled: false,
                    ..Default::default()
                },
                ..Default::default()
            },).insert_bundle(
                ColliderBundle {
                    shape: ColliderShape::cuboid(1., 1., 1.),
                    collider_type: ColliderType::Solid,
                    material: ColliderMaterial {
                        friction_combine_rule:  CoefficientCombineRule::Min,
                        ..Default::default()
                    },
                    flags: ColliderFlags {
                        collision_groups: InteractionGroups::new(masks.0,masks.1),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ).insert_bundle((
                ShipCell{
                    item: cell_data.item,
                    id: cell_id_int,
                    grid_type: GridMapType::Main,
                },
            ));

        }


        

    }

}

pub fn build_details1(
    current_map_details1_data : &Vec<CellDataWID>, 
    commands : &mut Commands, 
    gridmap_details1 : &mut ResMut<GridmapDetails1>,
) {
    
    for cell_data in current_map_details1_data.iter() {

        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let cell_id_int = Vec3Int{
            x: cell_id.x as i16,
            y: cell_id.y as i16,
            z: cell_id.z as i16,
        };

        gridmap_details1.data.insert(cell_id_int,
        CellData {
            item: cell_data.item,
            orientation: cell_data.orientation,
        });

        commands.spawn().insert_bundle((ShipCell{
            item: cell_data.item,
            id: cell_id_int,
            grid_type: GridMapType::Details1,
        },));


    }

}