use bevy::prelude::{Commands, Res};
use bevy_rapier3d::prelude::{CoefficientCombineRule, ColliderBundle, ColliderMaterial, ColliderShape, ColliderType, RigidBodyBundle, RigidBodyCcd, RigidBodyType};

use crate::space_core::{resources::all_ordered_cells::AllOrderedCells, systems::startup::launch_server::CellData};

use super::{gridmap_functions::cell_id_to_world, string_to_type_converters::string_vec3_to_vec3};


pub fn load_main_map_data(current_map_main_data : &Vec<CellData>, commands : &mut Commands, all_ordered_cells : &Res<AllOrderedCells>) {

    for cell_data in current_map_main_data.iter() {
        
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let world_position = cell_id_to_world(cell_id);

        if all_ordered_cells.main[((all_ordered_cells.main.len()-1) - cell_data.item as usize) as usize] == "securityCounter1" {

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
                    ..Default::default()
                }
            );

        } else {
            
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
                    ..Default::default()
                }
            );

        }


        

    }

}
