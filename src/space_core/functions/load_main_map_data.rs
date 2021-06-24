use bevy::prelude::{Commands, Res, ResMut};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderFlags, ColliderMaterial, ColliderShape, ColliderType, InteractionGroups, RigidBodyBundle, RigidBodyCcd, RigidBodyType};

use crate::space_core::{resources::{all_ordered_cells::AllOrderedCells, gridmap_main::{CellData, CellDataWID, GridmapMain}, precalculated_fov_data::Vec3Int}};

use super::{collider_interaction_groups::{ColliderGroup, get_bit_masks}, gridmap_functions::cell_id_to_world, string_to_type_converters::string_vec3_to_vec3};


pub fn load_main_map_data(
    current_map_main_data : &Vec<CellDataWID>, 
    commands : &mut Commands, 
    all_ordered_cells : &Res<AllOrderedCells>,
    gridmap_main : &mut ResMut<GridmapMain>,
) {



    for cell_data in current_map_main_data.iter() {
        
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let world_position = cell_id_to_world(Vec3Int{
            x: cell_id.x as i16,
            y: cell_id.y as i16,
            z: cell_id.z as i16,
        });

        gridmap_main.data.insert(Vec3Int {
            x: cell_id.x as i16,
            y: cell_id.y as i16,
            z: cell_id.z as i16,
        },
        CellData {
            item: cell_data.item,
            orientation: cell_data.orientation,
        });

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
            );

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
            );

        }


        

    }

}
