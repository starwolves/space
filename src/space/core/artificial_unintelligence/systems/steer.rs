use bevy_ecs::{
    entity::Entity,
    system::{Query, Res},
};
use bevy_math::{Vec2, Vec3};

use crate::space::core::{
    artificial_unintelligence::{
        components::{AiGoal, Blob, Path},
        functions::pathing_et_steering::{
            choose_vector, create_context_map, create_surroundings_map, get_proximity, get_vector,
        },
        resources::CONTEXT_MAP_RESOLUTION,
    },
    gridmap::{
        functions::gridmap_functions::world_to_cell_id,
        resources::{GridmapData, GridmapMain},
    },
    pawn::components::ControllerInput,
    rigid_body::components::CachedBroadcastTransform,
};

pub fn steer(
    mut ai_query: Query<(
        Entity,
        &AiGoal,
        &mut Blob,
        &mut Path,
        &CachedBroadcastTransform,
        &mut ControllerInput,
    )>,
    gridmap: Res<GridmapMain>,
    gridmap_data: Res<GridmapData>,
) -> () {
    for (_entity, goal, mut blob, mut path, transform, mut controller) in ai_query.iter_mut() {
        let mapped_vectors: [Vec2; 8] = [
            Vec2::new(0., -1.),
            Vec2::new(-0.70710677, -0.70710677),
            Vec2::new(-1., 0.),
            Vec2::new(-0.70710677, 0.70710677),
            Vec2::new(0., 1.),
            Vec2::new(0.70710677, 0.70710677),
            Vec2::new(1., 0.),
            Vec2::new(0.70710677, -0.70710677),
        ];
        let mut interest_map: [i32; CONTEXT_MAP_RESOLUTION] = [-1; CONTEXT_MAP_RESOLUTION];
        let mut danger_map: [i32; CONTEXT_MAP_RESOLUTION] = [-1; CONTEXT_MAP_RESOLUTION];
        let mut waypoints: [Option<Vec3>; 8] = [None; 8];
        let danger_waypoints: [Option<Vec3>; 8];
        let mut _chosen_vector = Vec2::ZERO;
        if let Some(path_waypoints) = &path.waypoints {
            let current_location = transform.transform.translation;
            let current_cell = world_to_cell_id(transform.transform.translation);
            waypoints[0] = Some(path_waypoints[0]);
            if get_proximity(waypoints[0].unwrap(), current_location) < 0.3 {
                path.update_waypoints();
            }
            danger_waypoints = create_surroundings_map(
                current_cell,
                0,
                &gridmap.grid_data,
                &gridmap_data.main_id_name_map,
            );
            interest_map =
                create_context_map(waypoints, current_location, goal.action, mapped_vectors);
            danger_map = create_context_map(
                danger_waypoints,
                current_location,
                goal.action,
                mapped_vectors,
            );

            _chosen_vector = choose_vector(interest_map, danger_map, mapped_vectors);
            controller.movement_vector = get_vector(waypoints[0].unwrap(), current_location);
        } else {
            controller.movement_vector = Vec2::new(0., 0.);
            path.remove_paths();
            _chosen_vector = Vec2::new(-10., -10.);
        }

        let old_position = blob.temp_position;
        blob.temp_position = world_to_cell_id(transform.transform.translation);

        if old_position != blob.temp_position && false {
            println!("final interest map: {:?}", interest_map);
            println!("danger map: {:?}", danger_map);
            println!("chosen vector: {:?}", _chosen_vector);
            blob.count = blob.count + 1;
            if blob.count % 3 == 0 {
                println!("{:?}), Position {:?}", blob.count, blob.temp_position);
            }
        }
    }
}
