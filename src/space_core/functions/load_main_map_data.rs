use bevy::prelude::Commands;
use bevy_rapier3d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};

use crate::space_core::systems::startup::launch_server::CellData;

use super::{gridmap_functions::cell_id_to_world, string_to_type_converters::string_vec3_to_vec3};


pub fn load_main_map_data(current_map_main_data : &Vec<CellData>, commands : &mut Commands) {

    for cell_data in current_map_main_data.iter() {
        
        let cell_id = string_vec3_to_vec3(&cell_data.id);

        let world_position = cell_id_to_world(cell_id);

        commands.spawn().insert_bundle((
            RigidBodyBuilder::new_static().translation(world_position.x, world_position.y, world_position.z),
            ColliderBuilder::cuboid(1., 1., 1.),
        ));

    }

}
