use std::fs;

use bevy::{prelude::*};

use bevy_rapier3d::{
    physics::{
        RapierPhysicsPlugin,
        RigidBodyHandleComponent
    },
    rapier::{
        dynamics::{
            RigidBodyBuilder,
            RigidBodySet
        },
        geometry::{
            ColliderBuilder
        },
        pipeline:: {
            PhysicsPipeline
        }
    }
};

use serde::{Deserialize};

struct PhysicsDynamicRigidBodyComponent;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(enable_physics_profiling.system())
        .add_startup_system(launch_server.system())
        .add_system(interpolate_entities_system.system())
        .run();
}

const DEFAULT_MAP_LOCATION : &str = "content\\maps\\bullseye\\main.json";

#[derive(Deserialize)]
struct CellData {
    id: String,
    item: i64,
    orientation: i64
}

fn launch_server(commands: &mut Commands) {

    // Load map json data

    let current_map_main_raw_json : String = fs::read_to_string(&DEFAULT_MAP_LOCATION).expect("main.rs launch_server() Error reading map main.json file from drive.");
    let current_map_main_data : Vec<CellData> = serde_json::from_str(&current_map_main_raw_json).expect("main.rs launch_server() Error parsing map main.json String.");
    
    for cell_data in current_map_main_data.iter() {
        
        let cell_id = json_vec3_to_vec3(&cell_data.id);

        let world_position = cell_id_to_world(cell_id);

        commands.spawn((
            RigidBodyBuilder::new_static().translation(world_position.x, world_position.y, world_position.z),
            ColliderBuilder::cuboid(1., 1., 1.),
        ));

    }
    
    commands.spawn((
        RigidBodyBuilder::new_dynamic().translation(0., 10., 0.),
        ColliderBuilder::cuboid(0.5, 0.5, 0.5),
        PhysicsDynamicRigidBodyComponent {}
    ));

}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn interpolate_entities_system(
    query: Query<
        (&RigidBodyHandleComponent, &PhysicsDynamicRigidBodyComponent)
    >,
    bodies: ResMut<RigidBodySet>
) {

    for (rigid_body_handle, _trait_dynamic_rigid_body) in query.iter() {
        

        
        if let Some(rigid_body) = bodies.get(rigid_body_handle.handle()) {
            
            info!("Dynamic cube is at {} !", rigid_body.position().translation);

        }

    }

}

const CELL_SIZE : f32 = 2.;
const Y_CENTER_OFFSET : f32 = 1.;

fn cell_id_to_world(cell_id : Vec3) -> Vec3 {

    let mut world_position : Vec3 = Vec3::zero();

    world_position.x = (cell_id.x * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.y = (cell_id.y * CELL_SIZE) + Y_CENTER_OFFSET;
    world_position.z = (cell_id.z * CELL_SIZE) + Y_CENTER_OFFSET;

    return world_position;

}

const STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE : &str = "main.rs json_vec3_to_vec3() Error cannot parse cell id string as Vector 3.";

fn json_vec3_to_vec3(string_vector : &str) -> Vec3 {

   

    let mut split_result : Vec<&str> = string_vector.split("(").collect();

    let mut new_string : &str = split_result[1];

    split_result = new_string.split(")").collect();

    new_string = split_result[0];

    split_result = new_string.split(",").collect();

    return Vec3::new(
        split_result[0].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[1].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE),
        split_result[2].parse::<f32>().expect(STRING_VEC3_TO_VEC3_CANNOT_PARSE_MESSAGE)
    );

}
