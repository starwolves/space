use bevy::prelude::{Commands, Res};
use bevy_rapier3d::prelude::{ColliderBundle, ColliderShape, RigidBodyBundle, RigidBodyType};

use crate::space_core::{resources::all_ordered_cells::AllOrderedCells, systems::startup::launch_server::CellData};

use super::{gridmap_functions::cell_id_to_world, string_to_type_converters::string_vec3_to_vec3};


pub fn load_main_map_data(current_map_main_data : &Vec<CellData>, commands : &mut Commands, all_ordered_cells : &Res<AllOrderedCells>) {

    for cell_data in current_map_main_data.iter() {
        
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let world_position = cell_id_to_world(cell_id);

        if all_ordered_cells.main[((all_ordered_cells.main.len()-1) - cell_data.item as usize) as usize] == "securityCounter1" {

            commands.spawn().insert_bundle((
                //RigidBodyBuilder::new_static().translation(world_position.x, world_position.y, world_position.z),
                RigidBodyBundle {
                    body_type: RigidBodyType::Static,
                    position: world_position.into(),
                    ..Default::default()
                },
                //ColliderBuilder::cuboid(1., 0.5, 0.5),
                ColliderBundle {
                    shape: ColliderShape::cuboid(1., 0.5, 0.5),
                    ..Default::default()
                }
            ));

        } else {

            commands.spawn().insert_bundle((
                //RigidBodyBuilder::new_static().translation(world_position.x, world_position.y, world_position.z),
                RigidBodyBundle {
                    body_type: RigidBodyType::Static,
                    position: world_position.into(),
                    ..Default::default()
                },
                //ColliderBuilder::cuboid(1., 1., 1.),
                ColliderBundle {
                    shape: ColliderShape::cuboid(1., 1., 1.),
                    ..Default::default()
                }
            ));

        }


        

    }

}
