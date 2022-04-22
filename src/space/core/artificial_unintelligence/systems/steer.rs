use bevy_ecs::{entity::Entity, system::Query};
use bevy_math::Vec2;

use crate::space::core::{
    artificial_unintelligence::{
        components::{AiGoal, Blob, Path},
        functions::steer::{get_proximity, get_vector},
    },
    // gridmap::functions::gridmap_functions::world_to_cell_id,
    pawn::components::ControllerInput,
    rigid_body::components::CachedBroadcastTransform,
};

pub fn steer(
    mut ai_query: Query<(
        Entity,
        &mut AiGoal,
        &mut Blob,
        &mut Path,
        &CachedBroadcastTransform,
        &mut ControllerInput,
    )>,
) -> () {
    for (_entity, _goal, mut _blob, mut path, transform, mut controller) in ai_query.iter_mut() {
        let _interest_map: [u32; 8];
        let _danger_map: [u32; 8];
        if let Some(waypoints) = &path.waypoints {
            let current_location = transform.transform.translation;
            let waypoint_target = waypoints[0];
            if get_proximity(waypoint_target, current_location) < 1.5 {
                path.update_waypoints();
            }

            controller.movement_vector = get_vector(waypoint_target, current_location);
        } else {
            controller.movement_vector = Vec2::new(0., 0.);
            path.remove_paths();
        }

        // let old_position = blob.temp_position;
        // blob.temp_position = world_to_cell_id(transform.transform.translation);

        // if old_position != blob.temp_position {
        //     blob.count = blob.count + 1;
        //     if blob.count % 3 == 0 {
        //         println!("{:?}), Position {:?}", blob.count, blob.temp_position);
        //     }
        // }
    }
}
