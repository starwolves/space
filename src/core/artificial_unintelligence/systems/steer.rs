use bevy_ecs::{
    entity::Entity,
    system::{Query, Res},
};
use bevy_math::{Vec2, Vec3};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::core::{
    artificial_unintelligence::{
        components::{Action, AiGoal, Blob, Path},
        functions::pathing_et_steering::{choose_vector, create_surroundings_map, get_proximity},
        resources::{ContextMapVectors, CONTEXT_MAP_RESOLUTION},
    },
    gridmap::{
        functions::gridmap_functions::world_to_cell_id,
        resources::{GridmapData, GridmapMain},
    },
    pawn::components::ControllerInput,
};

pub fn steer(
    mut ai_query: Query<(
        Entity,
        &AiGoal,
        &mut Blob,
        &mut Path,
        &RigidBodyPositionComponent,
        &mut ControllerInput,
    )>,
    gridmap: Res<GridmapMain>,
    gridmap_data: Res<GridmapData>,
    mapped_vectors: Res<ContextMapVectors>,
) -> () {
    for (_entity, goal, mut blob, mut path, rigid_body_position, mut controller) in
        ai_query.iter_mut()
    {
        if let Action::Standby = goal.action {
            continue;
        }
        let mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION] = mapped_vectors.context_map_vectors;
        let mut all_waypoints = Vec::new();
        let mut _chosen_vector = Vec2::ZERO;
        let mut current_location: Vec3 = rigid_body_position.position.translation.into();
        current_location.y = -1.;
        let current_cell = world_to_cell_id(current_location);
        for waypoint_option in create_surroundings_map(
            current_cell,
            0,
            &gridmap.grid_data,
            &gridmap_data.main_id_name_map,
        ) {
            if let Some(waypoint) = waypoint_option {
                all_waypoints.push(waypoint);
            }
        }
        match goal.action {
            Action::GoToPoint => {
                if let Some(path_waypoints) = &path.waypoints {
                    // Having this here might cause the AI to overshoot but I think its ok for now
                    all_waypoints.push(path_waypoints[0]);
                    if get_proximity(path_waypoints[0], current_location) < 1.0 {
                        path.update_waypoints();
                    }

                    controller.movement_vector =
                        choose_vector(all_waypoints, current_location, mapped_vectors);
                } else {
                    controller.movement_vector = Vec2::new(0., 0.);
                    path.remove_paths();
                }
            }
            _ => continue,
        }

        let old_position = blob.temp_position;
        blob.temp_position = world_to_cell_id(current_location);

        if old_position != blob.temp_position && false {
            println!("chosen vector: {:?}", _chosen_vector);
            blob.count = blob.count + 1;
            if blob.count % 3 == 0 {
                println!("{:?}), Position {:?}", blob.count, blob.temp_position);
            }
        }
    }
}
