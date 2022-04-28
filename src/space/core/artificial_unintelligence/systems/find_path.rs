use bevy_ecs::system::Query;
use bevy_ecs::{prelude::Entity, system::Res};
use rand::Rng;

use crate::space::core::artificial_unintelligence::components::{Action, AiGoal, Path};
use crate::space::core::artificial_unintelligence::functions::pathing_et_steering::generate_path_astar;
use crate::space::core::gridmap::functions::gridmap_functions::world_to_cell_id;
use crate::space::core::gridmap::resources::{GridmapData, GridmapMain, Vec3Int};
use crate::space::core::rigid_body::components::CachedBroadcastTransform;

pub fn find_path(
    mut ai_query: Query<(Entity, &AiGoal, &CachedBroadcastTransform, &mut Path)>,
    gridmap: Res<GridmapMain>,
    gridmap_data: Res<GridmapData>,
) {
    let location_list = [
        Vec3Int {
            x: 0,
            y: -1,
            z: -20,
        },
        Vec3Int {
            x: 100,
            y: -1,
            z: -4,
        },
        Vec3Int { x: 0, y: -1, z: 10 },
        Vec3Int {
            x: -42,
            y: -1,
            z: --2,
        },
        Vec3Int {
            x: 32,
            y: -1,
            z: -4,
        },
    ];
    for (_entity, goal, transform, mut path) in ai_query.iter_mut() {
        match goal.action {
            Action::PassiveStandby => {
                if let Some(_) = path.cell_path {
                    path.remove_paths();
                }
            }
            Action::AggressiveStandby => {
                if let Some(_) = path.cell_path {
                    path.remove_paths();
                }
            }
            Action::GoToPoint => {
                let current_location = world_to_cell_id(transform.transform.translation);
                let target_location = location_list[rand::thread_rng().gen_range(0..=4)];
                if let None = path.cell_path {
                    path.cell_path = generate_path_astar(
                        current_location,
                        target_location,
                        &gridmap.grid_data,
                        &gridmap_data.main_id_name_map,
                    );
                    path.create_waypoints();
                }
            }
        };
    }
}
