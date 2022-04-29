use bevy_math::{Vec2, Vec3};
use dotreds_binary_heap_plus::{BinaryHeap, MinComparator};
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
    f64::consts::PI,
};

use crate::core::{
    artificial_unintelligence::{
        components::{ContextMap, Waypoint, WaypointType},
        resources::CONTEXT_MAP_RESOLUTION,
    },
    gridmap::{
        functions::gridmap_functions::cell_id_to_world,
        resources::{CellData, Vec3Int, FOV_MAP_WIDTH},
    },
};

pub fn generate_path_astar(
    start_cell: Vec3Int,
    target_cell: Vec3Int,
    gridmap: &HashMap<Vec3Int, CellData>,
    main_id_name_map: &HashMap<i64, String>,
) -> Option<Vec<Vec3Int>> {
    let mut path_found = false;
    let mut open_set: BinaryHeap<PathNode, MinComparator> =
        BinaryHeap::with_capacity_min(FOV_MAP_WIDTH * FOV_MAP_WIDTH);
    let mut closed_set: Vec<PathNode> = Vec::new();
    open_set.push(PathNode::new(start_cell, target_cell, start_cell, 0));
    let mut path: Vec<Vec3Int> = Vec::new();
    let mut current_node: PathNode;

    while !path_found {
        if !open_set.is_empty() {
            current_node = open_set.pop().unwrap();
        } else {
            return None;
        }

        closed_set.push(current_node);

        if (current_node.id.x, current_node.id.z) == (target_cell.x, target_cell.z) {
            path = retrace_path(current_node, &closed_set, start_cell);
            path_found = true;
        }

        for neighbouring_cell in get_neighbouring_cells(current_node.id) {
            let mut skip = false;
            for node in &closed_set {
                if neighbouring_cell == node.id {
                    skip = true;
                    break;
                }
            }
            if skip {
                continue;
            }
            if !is_pathable(&neighbouring_cell, gridmap, main_id_name_map) {
                continue;
            }
            let neighbour_node = PathNode::new(
                neighbouring_cell,
                target_cell,
                current_node.id,
                current_node.gcost,
            );
            let mut already_in_open = false;
            for i in 0..open_set.len() {
                let node_id = open_set.data[i].id;
                if neighbour_node.gcost < open_set.data[i].gcost && neighbour_node.id == node_id {
                    open_set.data[i] = neighbour_node;
                    open_set.sift_up(0, i);
                    already_in_open = true;
                    break;
                }
                if neighbour_node.id == node_id {
                    already_in_open = true;
                    break;
                }
            }
            if !already_in_open {
                open_set.push(neighbour_node);
            }
        }
    }
    Some(path)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct PathNode {
    id: Vec3Int,
    parent: Vec3Int,
    gcost: i32,
    hcost: i32,
    fcost: i32,
}

impl PathNode {
    fn new(id: Vec3Int, target_cell: Vec3Int, parent: Vec3Int, parent_gcost: i32) -> PathNode {
        let hcost = get_distance(id, target_cell) as i32;
        let gcost = get_distance(id, parent) as i32 + parent_gcost;
        PathNode {
            id,
            gcost,
            hcost,
            parent,
            fcost: gcost + hcost,
        }
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fcost
            .cmp(&other.fcost)
            .then_with(|| self.hcost.cmp(&other.hcost))
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn retrace_path(
    current_node: PathNode,
    closed_set: &Vec<PathNode>,
    start_cell: Vec3Int,
) -> Vec<Vec3Int> {
    let mut path = VecDeque::new();
    let mut path_being_created = true;
    let mut next_node = current_node.parent;
    path.push_front(current_node.id);

    while path_being_created {
        for node in closed_set.iter().rev() {
            if next_node == start_cell {
                path.push_front(next_node);
                path_being_created = false;
                break;
            } else if node.id == next_node {
                path.push_front(node.id);
                next_node = node.parent;
            }
        }
    }
    Vec::from(path)
}

fn get_distance(cell_a: Vec3Int, cell_b: Vec3Int) -> i16 {
    const HORIZ_AND_VERT_COST: i16 = 10;
    const DIAGNOL_COST: i16 = 14;
    let x_diff = (cell_a.x - cell_b.x).abs();
    let y_diff = (cell_a.z - cell_b.z).abs();

    if x_diff > y_diff {
        return (y_diff * DIAGNOL_COST) + HORIZ_AND_VERT_COST * (x_diff - y_diff);
    } else {
        return (x_diff * DIAGNOL_COST) + HORIZ_AND_VERT_COST * (y_diff - x_diff);
    }
}

pub fn is_pathable(
    cell: &Vec3Int,
    gridmap: &HashMap<Vec3Int, CellData>,
    main_id_name_map: &HashMap<i64, String>,
) -> bool {
    let non_traversable_items = [
        "EMPTY0",
        "reinforcedGlassWall",
        "governmentDecoratedTable",
        "governmentWall",
        "bridgeDecoratedTable",
        "bridgeCounter",
        "bridgeWall",
        "securityDecoratedTable",
        "blackCellBlocking",
        "securityCounter1",
        "genericWall1",
        "blackCell",
        "securityWall",
    ];
    let above_cell = get_above_cell(&cell);
    match gridmap.get(cell) {
        Some(cell_data) => match main_id_name_map.get(&cell_data.item) {
            Some(item_name) => {
                if item_name == &"EMPTY0".to_string() {
                    return false;
                }
            }
            None => {
                return false;
            }
        },
        None => {
            return false;
        }
    }
    match gridmap.get(&above_cell) {
        Some(cell_data) => match main_id_name_map.get(&cell_data.item) {
            Some(item_name) => {
                if non_traversable_items.contains(&&item_name[..]) {
                    return false;
                }
            }
            None => (),
        },
        None => (),
    }
    true
}

pub fn get_neighbouring_cells(cell: Vec3Int) -> [Vec3Int; 8] {
    let neighbouring_cells: [Vec3Int; 8] = [
        Vec3Int {
            x: cell.x,
            y: -1,
            z: cell.z + 1,
        },
        Vec3Int {
            x: cell.x + 1,
            y: -1,
            z: cell.z + 1,
        },
        Vec3Int {
            x: cell.x + 1,
            y: -1,
            z: cell.z,
        },
        Vec3Int {
            x: cell.x + 1,
            y: -1,
            z: cell.z - 1,
        },
        Vec3Int {
            x: cell.x,
            y: -1,
            z: cell.z - 1,
        },
        Vec3Int {
            x: cell.x - 1,
            y: -1,
            z: cell.z - 1,
        },
        Vec3Int {
            x: cell.x - 1,
            y: -1,
            z: cell.z,
        },
        Vec3Int {
            x: cell.x - 1,
            y: -1,
            z: cell.z + 1,
        },
    ];

    neighbouring_cells
}

fn get_above_cell(cell: &Vec3Int) -> Vec3Int {
    Vec3Int {
        x: cell.x,
        y: (cell.y + 1),
        z: cell.z,
    }
}

pub fn get_vector(target: Waypoint, current: Vec3) -> Vec2 {
    let target = target.position;
    let x = target.x - current.x;
    let y = target.z - current.z;

    Vec2::new(x * -1., y).normalize_or_zero()
}

pub fn find_closest_two_vectors_and_strength(
    vector: Vec2,
    mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
) -> ((usize, i32), (usize, i32)) {
    let uniform_distance: f32 = mapped_vectors[0].distance(mapped_vectors[1]);
    let mut dist = vector.distance(mapped_vectors[0]);
    let mut closest = (0, dist);
    let mut next_closest = closest;
    for i in 0..mapped_vectors.len() {
        dist = vector.distance(mapped_vectors[i]);
        if dist < closest.1 {
            next_closest = closest;
            closest = (i, dist);
        } else if dist < next_closest.1 {
            next_closest = (i, dist);
        }
    }

    let closest_distribution = (closest.1 / uniform_distance) * 100.;
    let next_closest_distribution = (next_closest.1 / uniform_distance) * 100.;
    let closest_distribution = closest_distribution.round() as i32;
    let next_closest_distribution = next_closest_distribution.round() as i32;

    (
        (closest.0, closest_distribution),
        (next_closest.0, next_closest_distribution),
    )
}

pub fn find_closest_two_vectors_and_dot(
    vector: Vec2,
    mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
) -> ((usize, i32), (usize, i32)) {
    let mut dot = vector.dot(mapped_vectors[0]);
    let mut closest = (0, dot);
    let mut next_closest = closest;
    for i in 0..mapped_vectors.len() {
        dot = vector.dot(mapped_vectors[i]);
        if dot > closest.1 {
            next_closest = closest;
            closest = (i, dot);
        } else if dot > next_closest.1 {
            next_closest = (i, dot);
        }
    }
    closest.1 = (closest.1 * 100.).round();
    next_closest.1 = (next_closest.1 * 100.).round();
    (
        (closest.0, closest.1 as i32),
        (next_closest.0, next_closest.1 as i32),
    )
}

pub fn create_surroundings_map(
    current_cell: Vec3Int,
    _radius: i32,
    gridmap: &HashMap<Vec3Int, CellData>,
    main_id_name_map: &HashMap<i64, String>,
) -> [Option<Waypoint>; 8] {
    let mut blocked_waypoints = [None; 8];
    let neighbouring_cells = get_neighbouring_cells(current_cell);
    for i in 0..neighbouring_cells.len() {
        if !is_pathable(&neighbouring_cells[i], gridmap, main_id_name_map) {
            blocked_waypoints[i] = Some(Waypoint {
                position: cell_id_to_world(neighbouring_cells[i]),
                waypoint_type: WaypointType::CollisionObject,
            });
        }
    }
    blocked_waypoints
}

pub fn get_proximity(target: Waypoint, current: Vec3) -> f32 {
    let target = target.position;
    let x_dist = (target.x - current.x).abs();
    let y_dist = (target.z - current.z).abs();
    x_dist + y_dist
}

// A context map is an array of integers representing how desireable
// or undesirable the direction that corresponds with a given index is
pub fn choose_vector(
    waypoints: Vec<Waypoint>,
    current_location: Vec3,
    mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
) -> Vec2 {
    let mut interest_map = ContextMap::new_interest_map();
    let mut danger_map = ContextMap::new_danger_map();
    for waypoint in waypoints {
        match waypoint.waypoint_type {
            WaypointType::Pathing => {
                let vector = get_vector(waypoint, current_location);
                let mut new_interest_map = ContextMap::new_interest_map();
                new_interest_map.overwrite_all_slots_with_dot(vector, mapped_vectors);
                interest_map.combine(new_interest_map);
            }
            WaypointType::CollisionObject => {
                let vector = get_vector(waypoint, current_location);
                let (closest, _next_closest) =
                    find_closest_two_vectors_and_strength(vector, mapped_vectors);
                danger_map.write_to_slot(closest.0, closest.1);
            }
            _ => todo!(),
        }
    }
    interest_map.combine_with_danger(danger_map);
    mapped_vectors[interest_map.highest_value().0]
}

// You can use this to get an approximation of the original vector
// if you know that the context map slot values are both from dot products
// of that original vector and that slots vector
pub fn get_weighted_vector_dot(
    context_map_slot_a: (usize, i32),
    context_map_slot_b: (usize, i32),
    mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
) -> Vec2 {
    let uniform_dot_product = (mapped_vectors[0].dot(mapped_vectors[1]) * 100.).round() as i32;

    let a = context_map_slot_a.1 - uniform_dot_product;
    let b = context_map_slot_b.1 - uniform_dot_product;

    let total = (a + b) as f32;

    let weight_a = a as f32 / total;
    let weight_b = b as f32 / total;

    let mut weighted_vector = Vec2::ZERO;
    let mut vector_a = mapped_vectors[context_map_slot_a.0];
    let mut vector_b = mapped_vectors[context_map_slot_b.0];

    vector_a = Vec2::new(weight_a as f32 * vector_a.x, weight_a as f32 * vector_a.y);
    vector_b = Vec2::new(weight_b as f32 * vector_b.x, weight_b as f32 * vector_b.y);

    weighted_vector.x = vector_a.x + vector_b.x;
    weighted_vector.y = vector_a.y + vector_b.y;

    weighted_vector.normalize_or_zero()
}

// You can use this to get a vector that will fall inbetween
// two context map slots proportional to the values that were inside those context map slots
pub fn get_weighted_vector_strength(
    context_map_slot_a: (usize, i32),
    context_map_slot_b: (usize, i32),
    mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
) -> Vec2 {
    let mut weighted_vector = Vec2::ZERO;
    let mut vector_a = mapped_vectors[context_map_slot_a.0];
    let mut vector_b = mapped_vectors[context_map_slot_b.0];

    let total_strength = context_map_slot_a.1 + context_map_slot_b.1;
    let weight_a = context_map_slot_a.1 as f32 / total_strength as f32;
    let weight_b = context_map_slot_b.1 as f32 / total_strength as f32;

    vector_a = Vec2::new(weight_a * vector_a.x, weight_a * vector_a.y);
    vector_b = Vec2::new(weight_b * vector_b.x, weight_b * vector_b.y);

    weighted_vector.x = vector_a.x + vector_b.x;
    weighted_vector.y = vector_a.y + vector_b.y;

    weighted_vector.normalize_or_zero()
}

pub fn lowest_context_map_value(context_map: [i32; CONTEXT_MAP_RESOLUTION]) -> i32 {
    let mut lowest_value = context_map[0];
    for value in context_map {
        if value < lowest_value {
            lowest_value = value;
        }
    }
    lowest_value
}

pub fn highest_context_map_value(context_map: [i32; CONTEXT_MAP_RESOLUTION]) -> i32 {
    let mut highest_value = context_map[0];
    for value in context_map {
        if value > highest_value {
            highest_value = value;
        }
    }
    highest_value
}

pub fn build_mapped_vectors() -> [Vec2; CONTEXT_MAP_RESOLUTION] {
    let mut mapped_vectors: [Vec2; CONTEXT_MAP_RESOLUTION] = [Vec2::ZERO; 8];
    let starting_vec2 = Vec2::new(0., -1.);
    for i in 0..CONTEXT_MAP_RESOLUTION {
        let angle = i as f64 * 2. * PI / CONTEXT_MAP_RESOLUTION as f64;
        let x = (starting_vec2.x as f64 * angle.cos()) - (starting_vec2.y as f64 * angle.sin());
        let y = (starting_vec2.x as f64 * angle.sin()) + (starting_vec2.y as f64 * angle.cos());
        let x_before = x;
        let x_after = f64::trunc(x_before * 1000000.0).round() / 1000000.0;
        let y_before = y;
        let y_after = f64::trunc(y_before * 1000000.0).round() / 1000000.0;
        mapped_vectors[i] = Vec2::new(x_after as f32, y_after as f32).normalize_or_zero();
    }

    mapped_vectors
}
