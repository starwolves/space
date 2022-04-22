use dotreds_binary_heap_plus::{BinaryHeap, MinComparator};
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
};

use crate::space::core::gridmap::resources::{CellData, Vec3Int, FOV_MAP_WIDTH};

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
            if skip == true {
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

fn is_pathable(
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

fn get_neighbouring_cells(cell: Vec3Int) -> Vec<Vec3Int> {
    let mut neighbouring_cells = Vec::new();
    neighbouring_cells.push(Vec3Int {
        x: cell.x,
        y: -1,
        z: cell.z + 1,
    });
    neighbouring_cells.push(Vec3Int {
        x: cell.x + 1,
        y: -1,
        z: cell.z + 1,
    });
    neighbouring_cells.push(Vec3Int {
        x: cell.x + 1,
        y: -1,
        z: cell.z,
    });
    neighbouring_cells.push(Vec3Int {
        x: cell.x + 1,
        y: -1,
        z: cell.z - 1,
    });
    neighbouring_cells.push(Vec3Int {
        x: cell.x,
        y: -1,
        z: cell.z - 1,
    });
    neighbouring_cells.push(Vec3Int {
        x: cell.x - 1,
        y: -1,
        z: cell.z - 1,
    });
    neighbouring_cells.push(Vec3Int {
        x: cell.x - 1,
        y: -1,
        z: cell.z,
    });
    neighbouring_cells.push(Vec3Int {
        x: cell.x - 1,
        y: -1,
        z: cell.z + 1,
    });

    neighbouring_cells
}

fn get_above_cell(cell: &Vec3Int) -> Vec3Int {
    Vec3Int {
        x: cell.x,
        y: (cell.y + 1),
        z: cell.z,
    }
}
