use std::collections::HashMap;

use bevy::prelude::ResMut;
use resources::{grid::CellFace, math::Vec3Int};

use crate::grid::{CellTypeName, FullCell, Gridmap, GroupTypeId, GroupTypeName};

pub(crate) fn init_generic_wall_group(mut gridmap_data: ResMut<Gridmap>) {
    let mut wall_group = HashMap::new();
    let group_id = GroupTypeId(gridmap_data.group_type_incremental);
    gridmap_data.group_type_incremental += 1;
    wall_group.insert(
        Vec3Int { x: 0, y: 0, z: 0 },
        FullCell {
            face: CellFace::default(),
            orientation: 0,
            tile_type: *gridmap_data
                .main_name_id_map
                .get(&CellTypeName("generic_wall".to_string()))
                .unwrap(),
            entity_option: None,
        },
    );
    wall_group.insert(
        Vec3Int { x: 0, y: 1, z: 0 },
        FullCell {
            face: CellFace::default(),
            orientation: 0,
            tile_type: *gridmap_data
                .main_name_id_map
                .get(&CellTypeName("generic_wall".to_string()))
                .unwrap(),
            entity_option: None,
        },
    );
    gridmap_data.groups.insert(group_id, wall_group);
    gridmap_data
        .group_id_map
        .insert(GroupTypeName("generic_wall_group".to_string()), group_id);
    gridmap_data
        .id_group_map
        .insert(group_id, GroupTypeName("generic_wall_group".to_string()));
}
