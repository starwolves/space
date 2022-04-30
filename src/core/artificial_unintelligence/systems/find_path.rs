use bevy_ecs::system::{Commands, Query};
use bevy_ecs::{prelude::Entity, system::Res};
use bevy_rapier3d::prelude::RigidBodyPositionComponent;
use rand::Rng;

use crate::core::artificial_unintelligence::components::{Action, AiGoal, Path};
use crate::core::artificial_unintelligence::functions::pathing_et_steering::generate_path_astar;
use crate::core::gridmap::functions::gridmap_functions::world_to_cell_id;
use crate::core::gridmap::resources::{GridmapData, GridmapMain, Vec3Int};

pub fn find_path(
    mut ai_query: Query<(Entity, &AiGoal, &RigidBodyPositionComponent, &mut Path)>,
    gridmap: Res<GridmapMain>,
    gridmap_data: Res<GridmapData>,
    mut commands: Commands,
) {
    let location_list = [
        Vec3Int { x: -4, y: -1, z: 6 },
        Vec3Int {
            x: 2,
            y: -1,
            z: -35,
        },
        Vec3Int {
            x: -10,
            y: -1,
            z: -35,
        },
        Vec3Int {
            x: 4,
            y: -1,
            z: -20,
        },
        Vec3Int { x: 12, y: -1, z: 8 },
    ];
    for (_entity, goal, rigid_body_position, mut path) in ai_query.iter_mut() {
        match goal.action {
            Action::Standby => {
                if let Some(_) = path.cell_path {
                    path.remove_paths();
                }
            }
            Action::GoToPoint => {
                let current_location =
                    world_to_cell_id(rigid_body_position.position.translation.into());
                let target_location =
                    location_list[rand::thread_rng().gen_range(0..location_list.len())];
                if let None = path.cell_path {
                    path.cell_path = generate_path_astar(
                        current_location,
                        target_location,
                        &gridmap.grid_data,
                        &gridmap_data.main_id_name_map,
                    );
                    path.create_waypoints(&mut commands);
                }
            }
        };
    }
}
