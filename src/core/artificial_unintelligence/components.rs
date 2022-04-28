use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};

use crate::core::gridmap::{functions::gridmap_functions::cell_id_to_world, resources::Vec3Int};

#[derive(Copy, Clone)]
pub enum Action {
    GoToPoint,
    PassiveStandby,
    AggressiveStandby,
}

#[derive(Component)]
pub struct AiGoal {
    pub action: Action,
    pub location: Vec3Int,
}

impl Default for AiGoal {
    fn default() -> Self {
        Self {
            location: Vec3Int { x: 0, y: -1, z: 50 },
            action: Action::GoToPoint,
        }
    }
}

#[derive(Component)]
pub struct Path {
    pub cell_path: Option<Vec<Vec3Int>>,
    pub waypoints: Option<Vec<Vec3>>,
    pub waypoint_progress: usize,
}

impl Path {
    pub fn remove_paths(&mut self) {
        self.waypoints = None;
        self.cell_path = None;
        self.waypoint_progress = 0;
    }

    pub fn create_waypoints(&mut self) {
        const WAYPOINT_PRECISION: usize = 3; // A value of 1 results in maximum precision
        let mut waypoints_vec = Vec::new();
        let mut direction_old = Vec2::ZERO;
        let mut direction_new: Vec2;
        if let Some(cell_path) = &self.cell_path {
            waypoints_vec.push(cell_id_to_world(cell_path[0]));
            for i in 1..cell_path.len() {
                direction_new = Vec2::new(
                    cell_path[i - 1].x as f32 - cell_path[i].x as f32,
                    cell_path[i - 1].z as f32 - cell_path[i].z as f32,
                );
                if direction_old != direction_new {
                    waypoints_vec.push(cell_id_to_world(cell_path[i]));
                } else if i % WAYPOINT_PRECISION == 0 {
                    waypoints_vec.push(cell_id_to_world(cell_path[i]));
                }
                direction_old = direction_new;
            }
            if waypoints_vec[waypoints_vec.len() - 1]
                != cell_id_to_world(cell_path[cell_path.len() - 1])
            {
                waypoints_vec.push(cell_id_to_world(cell_path[cell_path.len() - 1]))
            }
        }
        if waypoints_vec.len() > 0 {
            self.waypoints = Some(waypoints_vec);
        }
    }

    pub fn update_waypoints(&mut self) {
        let mut new_waypoints: Vec<Vec3> = Vec::new();
        if let Some(waypoints) = &self.waypoints {
            if waypoints.len() == 1 {
                self.waypoints = None;
            } else {
                for i in 1..waypoints.len() {
                    new_waypoints.push(waypoints[i]);
                }
            }
        }
        if new_waypoints.len() > 0 {
            self.waypoints = Some(new_waypoints);
        }
    }
}

impl Default for Path {
    fn default() -> Self {
        Self {
            cell_path: None,
            waypoints: None,
            waypoint_progress: 0,
        }
    }
}

#[derive(Component)]
pub struct Blob {
    pub radius: u32,
    pub temp_position: Vec3Int,
    pub count: i32,
}

impl Default for Blob {
    fn default() -> Self {
        Self {
            radius: 10,
            temp_position: Vec3Int { x: 0, y: -1, z: 0 },
            count: 0,
        }
    }
}
