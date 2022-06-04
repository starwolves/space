use std::collections::HashMap;

use bevy_ecs::{prelude::Component, system::Commands};
use bevy_math::{Vec2, Vec3};
use bevy_transform::components::Transform;

use crate::{
    core::{
        entity::resources::SpawnData,
        gridmap::{functions::gridmap_functions::cell_id_to_world, resources::Vec3Int},
        networking::resources::ConsoleCommandVariantValues,
    },
};

use super::resources::CONTEXT_MAP_RESOLUTION;

#[derive(Copy, Clone)]
pub enum Action {
    GoToPoint,
    Standby,
}
#[derive(Copy, Clone)]
pub enum Stance {
    Aggresive,
    Passive,
}

#[derive(Component)]
pub struct AiGoal {
    pub action: Action,
    pub location: Vec3Int,
    pub stance: Stance,
}

impl Default for AiGoal {
    fn default() -> Self {
        Self {
            location: Vec3Int { x: 0, y: -1, z: 50 },
            action: Action::GoToPoint,
            stance: Stance::Passive,
        }
    }
}

#[derive(Component, Default)]
pub struct Path {
    pub cell_path: Option<Vec<Vec3Int>>,
    pub waypoints: Option<Vec<Waypoint>>,
    pub waypoint_progress: usize,
}

impl Path {
    pub fn remove_paths(&mut self) {
        self.waypoints = None;
        self.cell_path = None;
        self.waypoint_progress = 0;
    }

    pub fn create_waypoints(&mut self, commands: &mut Commands) {
        const PATH_PRECISION: usize = 3; // A value of 1 results in maximum precision
        let mut waypoints_vec = Vec::new();
        let mut direction_old = Vec2::ZERO;
        let mut direction_new: Vec2;
        if let Some(cell_path) = &self.cell_path {
            waypoints_vec.push(Waypoint::new(
                cell_id_to_world(cell_path[0]),
                WaypointType::Pathing,
            ));
            for i in 1..cell_path.len() {
                direction_new = Vec2::new(
                    cell_path[i - 1].x as f32 - cell_path[i].x as f32,
                    cell_path[i - 1].z as f32 - cell_path[i].z as f32,
                );
                if direction_old != direction_new {
                    LineArrowBundle::spawn(SpawnData {
                        entity_transform: Transform::from_translation(cell_id_to_world(
                            cell_path[i],
                        )),

                        commands,
                        correct_transform: true,
                        pawn_data_option: None,
                        held_data_option: None,
                        default_map_spawn: false,
                        properties: HashMap::from([(
                            "duration".to_string(),
                            ConsoleCommandVariantValues::Int(30),
                        )]),
                        showcase_data_option: &mut None,
                        entity_name: "lineArrow".to_string(),
                    });
                    waypoints_vec.push(Waypoint {
                        position: cell_id_to_world(cell_path[i]),
                        waypoint_type: WaypointType::Pathing,
                    });
                } else if i % PATH_PRECISION == 0 {
                    LineArrowBundle::spawn(SpawnData {
                        entity_transform: Transform::from_translation(cell_id_to_world(
                            cell_path[i],
                        )),

                        commands,
                        correct_transform: true,
                        pawn_data_option: None,
                        held_data_option: None,
                        default_map_spawn: false,
                        properties: HashMap::from([(
                            "duration".to_string(),
                            ConsoleCommandVariantValues::Int(30),
                        )]),
                        showcase_data_option: &mut None,
                        entity_name: "lineArrow".to_string(),
                    });
                    waypoints_vec.push(Waypoint {
                        position: cell_id_to_world(cell_path[i]),
                        waypoint_type: WaypointType::Pathing,
                    });
                }
                direction_old = direction_new;
            }
            if waypoints_vec[waypoints_vec.len() - 1].position
                != cell_id_to_world(cell_path[cell_path.len() - 1])
            {
                waypoints_vec.push(Waypoint {
                    position: cell_id_to_world(cell_path[cell_path.len() - 1]),
                    waypoint_type: WaypointType::Pathing,
                })
            }
        }
        if waypoints_vec.len() > 0 {
            self.waypoints = Some(waypoints_vec);
        }
    }

    pub fn update_waypoints(&mut self) {
        let mut new_waypoints: Vec<Waypoint> = Vec::new();
        if let Some(waypoints) = &mut self.waypoints {
            if waypoints.len() == 1 {
                self.waypoints = None;
            } else {
                new_waypoints = waypoints.drain(1..).collect();
            }
        }
        if new_waypoints.len() > 0 {
            self.waypoints = Some(new_waypoints);
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

#[derive(Copy, Clone, Debug)]
pub struct ContextMap {
    pub map: [i32; CONTEXT_MAP_RESOLUTION],
    pub is_danger: bool,
}

impl ContextMap {
    pub fn new_interest_map() -> Self {
        Self {
            map: [-100; CONTEXT_MAP_RESOLUTION],
            is_danger: false,
        }
    }

    pub fn new_danger_map() -> Self {
        Self {
            map: [0; CONTEXT_MAP_RESOLUTION],
            is_danger: true,
        }
    }

    pub fn combine(&mut self, other: ContextMap) {
        if self.is_danger == other.is_danger {
            for i in 0..other.map.len() {
                if other.map[i] > self.map[i] {
                    self.map[i] = other.map[i];
                }
            }
        } else {
            panic!("combine() only works if is_danger is the same on both maps")
        }
    }

    pub fn combine_with_danger(&mut self, danger_map: ContextMap) {
        if !self.is_danger && danger_map.is_danger {
            let (_, lowest_danger_value) = danger_map.lowest_value();
            for i in 0..danger_map.map.len() {
                if danger_map.map[i] > lowest_danger_value {
                    self.map[i] = -100;
                }
            }
        } else {
            panic!("combine_with_danger() only works when self is an interest map and danger_map is a danger map")
        }
    }

    pub fn write_to_slot(&mut self, index: usize, value: i32) {
        if self.map[index] < value {
            self.map[index] = value
        }
    }

    pub fn overwrite_to_slot(&mut self, index: usize, value: i32) {
        self.map[index] = value
    }

    // Uses the dot product of a given vector and each vector of a
    // context map to fill the values of a context map
    pub fn overwrite_all_slots_with_dot(
        &mut self,
        vector: Vec2,
        mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
    ) {
        for i in 0..mapped_vectors.len() {
            self.map[i] = (vector.dot(mapped_vectors[i]) * 100.).round() as i32;
        }
    }

    // overwrites 2 or less slots with the dot product of a given vector
    pub fn overwrite_2_slots_with_dot(
        mut self,
        vector: Vec2,
        mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
    ) {
        let mut map = self;
        for i in 0..mapped_vectors.len() {
            map.map[i] = (vector.dot(mapped_vectors[i]) * 100.).round() as i32;
        }

        let (greatest, next_greatest) = map.get_two_greatest_values();

        if greatest.1 >= 98 {
            self.map[greatest.0] = greatest.1;
        } else {
            self.map[greatest.0] = greatest.1;
            self.map[next_greatest.0] = next_greatest.1;
        }
        // writes 2 or less slots with the dot product of a given
        // vector if context map slot does not alredy contain a higher value
    }

    pub fn overwrite_slot_with_dot(
        mut self,
        vector: Vec2,
        mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
    ) {
        let mut map = self;
        for i in 0..mapped_vectors.len() {
            map.map[i] = (vector.dot(mapped_vectors[i]) * 100.).round() as i32;
        }

        let (index, highest_value) = map.highest_value();
        self.map[index] = highest_value;
    }

    pub fn lowest_value(&self) -> (usize, i32) {
        let mut lowest_value = (0, self.map[0]);
        for i in 0..self.map.len() {
            if self.map[i] < lowest_value.1 {
                lowest_value = (i, self.map[i]);
            }
        }
        lowest_value
    }

    pub fn highest_value(&self) -> (usize, i32) {
        let mut highest_value = (0, self.map[0]);
        for i in 0..self.map.len() {
            if self.map[i] > highest_value.1 {
                highest_value = (i, self.map[i]);
            }
        }
        highest_value
    }

    pub fn get_two_greatest_values(&self) -> ((usize, i32), (usize, i32)) {
        let mut greatest = (0, self.map[0]);
        let mut next_greatest = greatest;
        for i in 0..self.map.len() {
            let currently_testing = self.map[i];
            if currently_testing > greatest.1 {
                next_greatest = greatest;
                greatest = (i, currently_testing);
            } else if currently_testing > next_greatest.1 {
                next_greatest = (i, currently_testing);
            }
        }

        (greatest, next_greatest)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum WaypointType {
    Pathing,
    CollisionObject,
    Breadcrumb,
    Cover,
}

#[derive(Copy, Clone, Debug)]
pub struct Waypoint {
    pub position: Vec3,
    pub waypoint_type: WaypointType,
}

impl Waypoint {
    fn new(position: Vec3, waypoint_type: WaypointType) -> Waypoint {
        Waypoint {
            position,
            waypoint_type,
        }
    }
}
