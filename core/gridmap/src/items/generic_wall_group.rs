use std::collections::HashMap;

use bevy::prelude::ResMut;
use resources::math::Vec3Int;

use crate::grid::{CellTypeName, Gridmap, GroupTypeId, GroupTypeName};

pub(crate) fn init_wall_group_properties(mut gridmap_data: ResMut<Gridmap>) {
    let mut wall_group = HashMap::new();
    let group_id = GroupTypeId(0);
    wall_group.insert(
        Vec3Int { x: 0, y: 0, z: 0 },
        *gridmap_data
            .main_name_id_map
            .get(&CellTypeName("generic_wall_1".to_string()))
            .unwrap(),
    );
    wall_group.insert(
        Vec3Int { x: 0, y: 1, z: 0 },
        *gridmap_data
            .main_name_id_map
            .get(&CellTypeName("generic_wall_1".to_string()))
            .unwrap(),
    );
    gridmap_data.groups.insert(group_id, wall_group);
    gridmap_data
        .group_id_map
        .insert(GroupTypeName("generic_wall_group_1".to_string()), group_id);
    gridmap_data
        .id_group_map
        .insert(group_id, GroupTypeName("generic_wall_group_1".to_string()));
}
